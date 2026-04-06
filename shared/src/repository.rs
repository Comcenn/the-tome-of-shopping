use async_trait::async_trait;

use crate::item::Item;

#[async_trait]
pub trait ShoppingListRepository: Send + Sync {
    async fn list_items(&self) -> anyhow::Result<Vec<Item>>;
}
