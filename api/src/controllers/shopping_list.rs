use shared::{CreateItem, Item, ShoppingListRepository, item::UpdateItem, user::UserId};
use std::sync::Arc;

pub struct ShoppingListController<R>
where
    R: ShoppingListRepository + Clone + Send + Sync + 'static,
{
    repo: Arc<R>,
}

impl<R> ShoppingListController<R>
where
    R: ShoppingListRepository + Clone + Send + Sync + 'static,
{
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    pub async fn list_items(&self, user: UserId, password: &str) -> anyhow::Result<Vec<Item>> {
        self.repo.list_items(user, password).await
    }

    pub async fn add_item(
        &self,
        item: CreateItem,
        user: UserId,
        password: &str,
    ) -> anyhow::Result<()> {
        self.repo.add_item(item, user, password).await?;
        Ok(())
    }

    pub async fn remove_item(
        &self,
        item_id: i32,
        quantity: i32,
        user: UserId,
        password: &str,
    ) -> anyhow::Result<()> {
        self.repo
            .remove_item(item_id, quantity, user, password)
            .await?;
        Ok(())
    }

    pub async fn update_item(
        &self,
        item_id: i32,
        item: UpdateItem,
        user: UserId,
        password: &str,
    ) -> anyhow::Result<()> {
        self.repo.update_item(item_id, item, user, password).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::dec;
    use shared::{CreateItem, InMemoryRepo, item::UpdateItem, user::UserId};
    use std::sync::Arc;

    fn user(name: &str) -> UserId {
        UserId(name.to_string())
    }

    const PW: &str = "pw";

    #[tokio::test]
    async fn controller_lists_items_for_user() {
        let repo = Arc::new(InMemoryRepo::new());
        let controller = ShoppingListController::new(repo.clone());

        controller
            .add_item(CreateItem::new("Milk", dec!(1.20), 1), user("alice"), PW)
            .await
            .unwrap();

        let items = controller.list_items(user("alice"), PW).await.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "Milk");
    }

    #[tokio::test]
    async fn controller_isolates_users() {
        let repo = Arc::new(InMemoryRepo::new());
        let controller = ShoppingListController::new(repo.clone());

        controller
            .add_item(CreateItem::new("Milk", dec!(1.20), 1), user("alice"), PW)
            .await
            .unwrap();

        controller
            .add_item(CreateItem::new("Bread", dec!(0.95), 1), user("bob"), PW)
            .await
            .unwrap();

        let alice_items = controller.list_items(user("alice"), PW).await.unwrap();
        let bob_items = controller.list_items(user("bob"), PW).await.unwrap();

        assert_eq!(alice_items.len(), 1);
        assert_eq!(bob_items.len(), 1);

        assert_eq!(alice_items[0].name, "Milk");
        assert_eq!(bob_items[0].name, "Bread");
    }

    #[tokio::test]
    async fn controller_updates_items() {
        let repo = Arc::new(InMemoryRepo::new());
        let controller = ShoppingListController::new(repo.clone());
        let u = user("carol");

        controller
            .add_item(CreateItem::new("Eggs", dec!(2.50), 1), u.clone(), PW)
            .await
            .unwrap();

        let items = controller.list_items(u.clone(), PW).await.unwrap();
        let id = items[0].id;

        controller
            .update_item(id, UpdateItem::PickedUp { picked_up: true }, u.clone(), PW)
            .await
            .unwrap();

        let updated = controller.list_items(u, PW).await.unwrap();
        assert!(updated[0].picked_up);
    }

    #[tokio::test]
    async fn controller_removes_items() {
        let repo = Arc::new(InMemoryRepo::new());
        let controller = ShoppingListController::new(repo.clone());
        let u = user("dave");

        controller
            .add_item(CreateItem::new("Juice", dec!(1.50), 2), u.clone(), PW)
            .await
            .unwrap();

        let items = controller.list_items(u.clone(), PW).await.unwrap();
        let id = items[0].id;

        controller.remove_item(id, 2, u.clone(), PW).await.unwrap();

        let after = controller.list_items(u, PW).await.unwrap();
        assert!(after.is_empty());
    }
}
