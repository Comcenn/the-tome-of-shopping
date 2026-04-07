use async_trait::async_trait;

use crate::{CreateItem, item::Item};

#[async_trait]
pub trait ShoppingListRepository: Send + Sync {
    async fn list_items(&self) -> anyhow::Result<Vec<Item>>;

    async fn add_item(&self, item: CreateItem) -> anyhow::Result<()>;
}
