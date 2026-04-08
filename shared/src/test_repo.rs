use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{CreateItem, Item, ShoppingListRepository, item::UpdateItem, user::UserId};

#[derive(Clone)]
pub struct InMemoryRepo {
    // Each user has their own list of items
    data: Arc<Mutex<HashMap<UserId, Vec<Item>>>>,
}

impl InMemoryRepo {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl ShoppingListRepository for InMemoryRepo {
    async fn list_items(&self, user: UserId, _password: &str) -> anyhow::Result<Vec<Item>> {
        let data = self.data.lock().await;
        Ok(data.get(&user).cloned().unwrap_or_default())
    }

    async fn add_item(
        &self,
        item: CreateItem,
        user: UserId,
        _password: &str,
    ) -> anyhow::Result<()> {
        let mut data = self.data.lock().await;
        let entry = data.entry(user).or_default();

        // Generate a stable ID (1-based)
        let id = (entry.len() as i32) + 1;
        let item_order = (entry.len() as i32) + 1;

        entry.push(Item {
            id,
            name: item.name,
            price: item.price,
            item_order: item_order,
            quantity: item.quantity,
            picked_up: false,
        });

        Ok(())
    }

    async fn remove_item(
        &self,
        item_id: i32,
        _quantity: i32,
        user: UserId,
        _password: &str,
    ) -> anyhow::Result<()> {
        let mut data = self.data.lock().await;

        if let Some(items) = data.get_mut(&user) {
            // If quantity matches, remove the item entirely
            items.retain(|i| i.id != item_id);
        }

        Ok(())
    }

    async fn update_item(
        &self,
        item_id: i32,
        update: UpdateItem,
        user: UserId,
        _password: &str,
    ) -> anyhow::Result<()> {
        let mut data = self.data.lock().await;

        if let Some(items) = data.get_mut(&user) {
            if let Some(item) = items.iter_mut().find(|i| i.id == item_id) {
                match update {
                    UpdateItem::PickedUp { picked_up } => {
                        item.picked_up = picked_up;
                    }
                    UpdateItem::ItemOrder { item_order } => {
                        item.item_order = item_order;
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::dec;

    use super::*;

    fn user(name: &str) -> UserId {
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
    async fn repo_isolates_users() {
        let repo = InMemoryRepo::new();

        // Alice adds one item
        repo.add_item(sample_item("Milk"), user("alice"), "")
            .await
            .unwrap();

        // Bob adds one item
        repo.add_item(sample_item("Bread"), user("bob"), "")
            .await
            .unwrap();

        // Alice should only see her item
        let alice_items = repo.list_items(user("alice"), "").await.unwrap();
        assert_eq!(alice_items.len(), 1);
        assert_eq!(alice_items[0].name, "Milk");

        // Bob should only see his item
        let bob_items = repo.list_items(user("bob"), "").await.unwrap();
        assert_eq!(bob_items.len(), 1);
        assert_eq!(bob_items[0].name, "Bread");
    }

    #[tokio::test]
    async fn add_item_assigns_incrementing_ids_per_user() {
        let repo = InMemoryRepo::new();

        repo.add_item(sample_item("A"), user("alice"), "")
            .await
            .unwrap();
        repo.add_item(sample_item("B"), user("alice"), "")
            .await
            .unwrap();
        repo.add_item(sample_item("C"), user("alice"), "")
            .await
            .unwrap();

        let items = repo.list_items(user("alice"), "").await.unwrap();
        assert_eq!(items.len(), 3);

        assert_eq!(items[0].id, 1);
        assert_eq!(items[1].id, 2);
        assert_eq!(items[2].id, 3);
    }

    #[tokio::test]
    async fn remove_item_removes_only_target_item() {
        let repo = InMemoryRepo::new();

        repo.add_item(sample_item("A"), user("alice"), "")
            .await
            .unwrap();
        repo.add_item(sample_item("B"), user("alice"), "")
            .await
            .unwrap();
        repo.add_item(sample_item("C"), user("alice"), "")
            .await
            .unwrap();

        // Remove item with ID 2
        repo.remove_item(2, 1, user("alice"), "").await.unwrap();

        let items = repo.list_items(user("alice"), "").await.unwrap();
        assert_eq!(items.len(), 2);

        let names: Vec<_> = items.iter().map(|i| i.name.as_str()).collect();
        assert_eq!(names, vec!["A", "C"]);
    }

    #[tokio::test]
    async fn update_item_updates_picked_up_flag() {
        let repo = InMemoryRepo::new();

        repo.add_item(sample_item("Milk"), user("alice"), "")
            .await
            .unwrap();

        repo.update_item(
            1,
            UpdateItem::PickedUp { picked_up: true },
            user("alice"),
            "",
        )
        .await
        .unwrap();

        let items = repo.list_items(user("alice"), "").await.unwrap();
        assert_eq!(items[0].picked_up, true);
    }

    #[tokio::test]
    async fn update_item_updates_item_order() {
        let repo = InMemoryRepo::new();

        repo.add_item(sample_item("Milk"), user("alice"), "")
            .await
            .unwrap();

        repo.update_item(
            1,
            UpdateItem::ItemOrder { item_order: 42 },
            user("alice"),
            "",
        )
        .await
        .unwrap();

        let items = repo.list_items(user("alice"), "").await.unwrap();
        assert_eq!(items[0].item_order, 42);
    }

    #[tokio::test]
    async fn removing_item_for_one_user_does_not_affect_other_users() {
        let repo = InMemoryRepo::new();

        repo.add_item(sample_item("A"), user("alice"), "")
            .await
            .unwrap();
        repo.add_item(sample_item("B"), user("bob"), "")
            .await
            .unwrap();

        // Remove Alice's item
        repo.remove_item(1, 1, user("alice"), "").await.unwrap();

        // Bob's item must remain untouched
        let bob_items = repo.list_items(user("bob"), "").await.unwrap();
        assert_eq!(bob_items.len(), 1);
        assert_eq!(bob_items[0].name, "B");
    }
}
