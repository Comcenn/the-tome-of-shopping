use std::{fs::File, path::PathBuf};

use anyhow::Ok;
use async_trait::async_trait;
use fs2::FileExt;
use rust_decimal::dec;
use shared::{CreateItem, Item, ShoppingListRepository};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct FakeRepo;

#[async_trait]
impl ShoppingListRepository for FakeRepo {
    async fn list_items(&self) -> anyhow::Result<Vec<Item>> {
        Ok(vec![
            Item::new(1, 1, "Milk", dec!(1.20), 1),
            Item::new(2, 2, "Bread", dec!(0.95), 1),
            Item::new(3, 3, "Eggs", dec!(2.50), 1),
        ])
    }

    async fn add_item(&self, item: CreateItem) -> anyhow::Result<()> {
        Ok(())
    }
}

pub struct FileBackedStore {
    path: PathBuf,
    lock_path: PathBuf,
    items: RwLock<Vec<Item>>,
}

impl FileBackedStore {
    pub async fn new(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let lock_path = path.with_extension("lock");

        // Ensure lock file exists
        let _ = File::create(&lock_path);

        // Load initial items
        let items = match tokio::fs::read(&path).await {
            std::result::Result::Ok(bytes) => serde_json::from_slice(&bytes).unwrap_or_default(),
            Err(_) => Vec::new(),
        };

        Self {
            path,
            lock_path,
            items: RwLock::new(items),
        }
    }

    /// Acquire exclusive lock, read file fresh, return Vec<Item>
    async fn load_fresh(&self) -> Vec<Item> {
        let file = File::open(&self.lock_path).expect("lock file missing");
        file.lock_exclusive().expect("failed to lock");

        let result = match std::fs::read(&self.path) {
            std::result::Result::Ok(bytes) => serde_json::from_slice(&bytes).unwrap_or_default(),
            Err(_) => Vec::new(),
        };

        file.unlock().expect("failed to unlock");
        result
    }

    /// Write items atomically with exclusive lock
    async fn persist(&self, items: &[Item]) {
        let file = File::open(&self.lock_path).expect("lock file missing");
        file.lock_exclusive().expect("failed to lock");

        let tmp = self.path.with_extension("tmp");
        let bytes = serde_json::to_vec_pretty(items).unwrap();

        std::fs::write(&tmp, bytes).expect("failed to write tmp file");
        std::fs::rename(&tmp, &self.path).expect("failed to rename tmp file");

        file.unlock().expect("failed to unlock");
    }
}

impl Clone for FileBackedStore {
    fn clone(&self) -> Self {
        let items = {
            let guard = self.items.blocking_read();
            guard.clone()
        };

        Self {
            path: self.path.clone(),
            lock_path: self.lock_path.clone(),
            items: RwLock::new(items),
        }
    }
}

#[async_trait]
impl ShoppingListRepository for FileBackedStore {
    async fn add_item(&self, item: CreateItem) -> anyhow::Result<()> {
        // Always load fresh state
        let mut items = self.load_fresh().await;

        if let Some(existing) = items.iter_mut().find(|i| i.name == item.name) {
            existing.quantity += item.quantity;
        } else {

            let next_id = items.iter().map(|i| i.id).max().unwrap_or(0) + 1;
            let next_order = items.iter().map(|i| i.item_order).max().unwrap_or(0) + 1;
            
            let new_item = Item::new(next_id, next_order, item.name, item.price, item.quantity);

            items.push(new_item);
        }

        // Persist atomically
        self.persist(&items).await;

        // Update in-memory cache
        let mut cache = self.items.write().await;
        *cache = items;

        Ok(())
    }

    async fn list_items(&self) -> anyhow::Result<Vec<Item>> {
        // Always load fresh state
        let fresh = self.load_fresh().await;

        // Update cache
        let mut cache = self.items.write().await;
        *cache = fresh.clone();

        Ok(fresh)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::dec;
    use tempfile::tempdir;
    use tokio::fs;

    #[tokio::test]
    async fn test_list_items_returns_fake_data() {
        let repo = FakeRepo;

        let items = repo.list_items().await.unwrap();

        assert_eq!(items.len(), 3);
        assert_eq!(items[0].name, "Milk");
        assert_eq!(items[0].price, dec!(1.20));
    }

    fn sample_item(id: i32) -> Item {
        Item::new(id, id, format!("Item{}", id), dec!(12.30), 1)
    }

    fn sample_create_item(id: i32) -> CreateItem {
        CreateItem::new(format!("Item{}", id),dec!(10.99), 1)
    }

    #[tokio::test]
    async fn new_creates_lock_file_and_loads_empty_when_missing() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("store.json");

        let store = FileBackedStore::new(&path).await;

        // Lock file should exist
        let lock_path = path.with_extension("lock");
        assert!(lock_path.exists());

        // No data yet
        let items = store.list_items().await.unwrap();
        assert!(items.is_empty());
    }

    #[tokio::test]
    async fn new_loads_existing_json_items() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("store.json");

        let initial = vec![sample_item(1), sample_item(2)];
        fs::write(&path, serde_json::to_vec(&initial).unwrap())
            .await
            .unwrap();

        let store = FileBackedStore::new(&path).await;

        let items = store.list_items().await.unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].name, "Item1");
    }

    #[tokio::test]
    async fn add_item_persists_and_updates_cache() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("store.json");

        let store = FileBackedStore::new(&path).await;

        store.add_item(sample_create_item(10)).await.unwrap();

        // Read from disk directly
        let bytes = fs::read(&path).await.unwrap();
        let disk_items: Vec<Item> = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(disk_items.len(), 1);
        assert_eq!(disk_items[0].id, 1);

        // Read via repo
        let listed = store.list_items().await.unwrap();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].id, 1);
    }

    #[tokio::test]
    async fn list_items_always_reads_fresh_state() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("store.json");

        let store = FileBackedStore::new(&path).await;

        // Write directly to disk, bypassing cache
        let injected = vec![sample_item(99)];
        fs::write(&path, serde_json::to_vec(&injected).unwrap())
            .await
            .unwrap();

        let items = store.list_items().await.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, 99);
    }

    #[tokio::test]
    async fn persist_writes_atomically_via_tmp_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("store.json");
        let tmp_path = path.with_extension("tmp");

        let store = FileBackedStore::new(&path).await;

        store.add_item(sample_create_item(1)).await.unwrap();

        // tmp file should not remain after atomic rename
        assert!(!tmp_path.exists());

        // final file should contain the item
        let bytes = fs::read(&path).await.unwrap();
        let items: Vec<Item> = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(items.len(), 1);
    }

    #[tokio::test]
    async fn load_fresh_respects_lock_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("store.json");

        let store = FileBackedStore::new(&path).await;

        // Write something to disk
        let initial = vec![sample_item(5)];
        fs::write(&path, serde_json::to_vec(&initial).unwrap())
            .await
            .unwrap();

        // load_fresh is private, so call via list_items
        let items = store.list_items().await.unwrap();

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, 5);

        // Lock file should still exist
        assert!(path.with_extension("lock").exists());
    }
}
