use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Result, anyhow};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::fs;

use async_trait::async_trait;
use shared::{CreateItem, Item, item::UpdateItem, user::UserId};

#[derive(Clone)]
pub struct FileBackedStore {
    base_dir: Arc<PathBuf>,
}

impl FileBackedStore {
    pub async fn new(base_dir: impl AsRef<Path>) -> Self {
        let path = base_dir.as_ref().to_path_buf();
        fs::create_dir_all(&path).await.unwrap();
        Self {
            base_dir: Arc::new(path),
        }
    }
}

/* -------------------------
PASSWORD + SALT HANDLING
------------------------- */

#[derive(Serialize, Deserialize)]
struct UserMeta {
    salt: String,
    password_hash: String,
}

fn generate_salt() -> String {
    let mut bytes = [0u8; 16];
    rand::rng().fill_bytes(&mut bytes);
    hex::encode(bytes)
}

fn hash_password(username: &str, password: &str, salt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("{username}:{password}:{salt}"));
    let hash = hasher.finalize();
    hex::encode(hash)
}

/* -------------------------
METADATA FILE HANDLING
------------------------- */

impl FileBackedStore {
    fn meta_file(&self, username: &str) -> PathBuf {
        self.base_dir.join(format!("{username}.meta.json"))
    }

    async fn load_or_create_meta(&self, username: &str, password: &str) -> Result<UserMeta> {
        let meta_path = self.meta_file(username);

        if meta_path.exists() {
            // Load existing metadata
            let data = fs::read_to_string(&meta_path).await?;
            let meta: UserMeta = serde_json::from_str(&data)?;

            // Verify password
            let expected = hash_password(username, password, &meta.salt);
            if expected != meta.password_hash {
                return Err(anyhow!("Invalid username or password"));
            }

            Ok(meta)
        } else {
            // Create new metadata
            let salt = generate_salt();
            let password_hash = hash_password(username, password, &salt);

            let meta = UserMeta {
                salt,
                password_hash,
            };
            let json = serde_json::to_string_pretty(&meta)?;
            fs::write(meta_path, json).await?;

            Ok(meta)
        }
    }
}

/* -------------------------
USER DATA FILE HANDLING
------------------------- */

impl FileBackedStore {
    fn user_file(&self, username: &str, password_hash: &str) -> PathBuf {
        self.base_dir
            .join(format!("{username}__{password_hash}.json"))
    }
}

/* -------------------------
REPOSITORY IMPLEMENTATION
------------------------- */

#[async_trait]
impl shared::ShoppingListRepository for FileBackedStore {
    async fn list_items(&self, user: UserId, password: &str) -> Result<Vec<Item>> {
        let meta = self.load_or_create_meta(&user.0, password).await?;
        let path = self.user_file(&user.0, &meta.password_hash);

        if !path.exists() {
            return Ok(vec![]);
        }

        let data = fs::read_to_string(path).await?;
        let items: Vec<Item> = serde_json::from_str(&data)?;
        Ok(items)
    }

    async fn add_item(&self, item: CreateItem, user: UserId, password: &str) -> Result<()> {
        let meta = self.load_or_create_meta(&user.0, password).await?;
        let path = self.user_file(&user.0, &meta.password_hash);

        let mut items = if path.exists() {
            let data = fs::read_to_string(&path).await?;
            serde_json::from_str(&data)?
        } else {
            Vec::<Item>::new()
        };

        let id = (items.len() as i32) + 1;
        let item_order = id;

        items.push(Item {
            id,
            name: item.name,
            price: item.price,
            item_order,
            quantity: item.quantity,
            picked_up: false,
        });

        let json = serde_json::to_string_pretty(&items)?;
        fs::write(path, json).await?;

        Ok(())
    }

    async fn remove_item(
        &self,
        item_id: i32,
        _qty: i32,
        user: UserId,
        password: &str,
    ) -> Result<()> {
        let meta = self.load_or_create_meta(&user.0, password).await?;
        let path = self.user_file(&user.0, &meta.password_hash);

        if !path.exists() {
            return Ok(());
        }

        let data = fs::read_to_string(&path).await?;
        let mut items: Vec<Item> = serde_json::from_str(&data)?;

        items.retain(|i| i.id != item_id);

        let json = serde_json::to_string_pretty(&items)?;
        fs::write(path, json).await?;

        Ok(())
    }

    async fn update_item(
        &self,
        item_id: i32,
        update: UpdateItem,
        user: UserId,
        password: &str,
    ) -> Result<()> {
        let meta = self.load_or_create_meta(&user.0, password).await?;
        let path = self.user_file(&user.0, &meta.password_hash);

        if !path.exists() {
            return Ok(());
        }

        let data = fs::read_to_string(&path).await?;
        let mut items: Vec<Item> = serde_json::from_str(&data)?;

        if let Some(item) = items.iter_mut().find(|i| i.id == item_id) {
            match update {
                UpdateItem::PickedUp { picked_up } => item.picked_up = picked_up,
                UpdateItem::ItemOrder { item_order } => item.item_order = item_order,
            }
        }

        let json = serde_json::to_string_pretty(&items)?;
        fs::write(path, json).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::dec;
    use shared::{CreateItem, ShoppingListRepository, item::UpdateItem, user::UserId};
    use tempfile::tempdir;

    fn sample_user(name: &str) -> UserId {
        UserId(name.to_string())
    }

    fn sample_item(name: &str) -> CreateItem {
        CreateItem {
            name: name.to_string(),
            price: dec!(1.0),
            quantity: 1,
        }
    }

    #[tokio::test]
    async fn creates_metadata_on_first_use() {
        let dir = tempdir().unwrap();
        let store = FileBackedStore::new(dir.path()).await;

        let username = sample_user("alice");
        let test_password = "secret";

        let _ = store.list_items(username, test_password).await.unwrap();

        let meta_path = dir.path().join("alice.meta.json");
        assert!(meta_path.exists(), "metadata file should be created");
    }

    #[tokio::test]
    async fn rejects_wrong_password() {
        let dir = tempdir().unwrap();
        let store = FileBackedStore::new(dir.path()).await;

        let user = sample_user("bob");

        // Create metadata with correct password
        let _ = store.list_items(user.clone(), "correct").await.unwrap();

        // Wrong password should fail
        let err = store.list_items(user.clone(), "wrong").await.unwrap_err();
        assert!(err.to_string().contains("Invalid username or password"));
    }

    #[tokio::test]
    async fn adds_and_lists_items() {
        let dir = tempdir().unwrap();
        let store = FileBackedStore::new(dir.path()).await;

        let user = sample_user("carol");
        let password = "pw123";

        store
            .add_item(sample_item("Milk"), user.clone(), password)
            .await
            .unwrap();
        store
            .add_item(sample_item("Bread"), user.clone(), password)
            .await
            .unwrap();

        let items = store.list_items(user.clone(), password).await.unwrap();

        assert_eq!(items.len(), 2);
        assert_eq!(items[0].name, "Milk");
        assert_eq!(items[1].name, "Bread");
    }

    #[tokio::test]
    async fn items_are_isolated_per_user_and_password() {
        let dir = tempdir().unwrap();
        let store = FileBackedStore::new(dir.path()).await;

        let alice = sample_user("alice");
        let bob = sample_user("bob");

        store
            .add_item(sample_item("Milk"), alice.clone(), "a1")
            .await
            .unwrap();
        store
            .add_item(sample_item("Bread"), bob.clone(), "b1")
            .await
            .unwrap();

        let alice_items = store.list_items(alice.clone(), "a1").await.unwrap();
        let bob_items = store.list_items(bob.clone(), "b1").await.unwrap();

        assert_eq!(alice_items.len(), 1);
        assert_eq!(alice_items[0].name, "Milk");

        assert_eq!(bob_items.len(), 1);
        assert_eq!(bob_items[0].name, "Bread");
    }

    #[tokio::test]
    async fn update_item_changes_fields() {
        let dir = tempdir().unwrap();
        let store = FileBackedStore::new(dir.path()).await;

        let user = sample_user("dave");
        let password = "pw";

        store
            .add_item(sample_item("Eggs"), user.clone(), password)
            .await
            .unwrap();

        store
            .update_item(
                1,
                UpdateItem::PickedUp { picked_up: true },
                user.clone(),
                password,
            )
            .await
            .unwrap();

        let items = store.list_items(user.clone(), password).await.unwrap();
        assert_eq!(items[0].picked_up, true);
    }

    #[tokio::test]
    async fn remove_item_deletes_correct_entry() {
        let dir = tempdir().unwrap();
        let store = FileBackedStore::new(dir.path()).await;

        let user = sample_user("eve");
        let password = "pw";

        store
            .add_item(sample_item("Milk"), user.clone(), password)
            .await
            .unwrap();
        store
            .add_item(sample_item("Bread"), user.clone(), password)
            .await
            .unwrap();

        store
            .remove_item(1, 1, user.clone(), password)
            .await
            .unwrap();

        let items = store.list_items(user.clone(), password).await.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "Bread");
    }
}
