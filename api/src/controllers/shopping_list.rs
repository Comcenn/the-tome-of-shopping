use std::sync::Arc;

use shared::{CreateItem, Item, ShoppingListRepository};

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

    pub async fn list_items(&self) -> anyhow::Result<Vec<Item>> {
        self.repo.list_items().await
    }

    pub async fn add_item(&self, item: CreateItem) -> anyhow::Result<()> {
        self.repo.add_item(item).await?;
        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::repositories::FakeRepo;

    #[tokio::test]
    async fn test_controller_returns_list_of_items() {
        let repo = Arc::new(FakeRepo);

        let controller = ShoppingListController::new(repo);

        let items = controller.list_items().await.unwrap();

        assert_eq!(items.len(), 3)
    }
}
