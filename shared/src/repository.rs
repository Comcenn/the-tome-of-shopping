use async_trait::async_trait;

use crate::{
    CreateItem,
    item::{Item, UpdateItem},
};

#[async_trait]
pub trait ShoppingListRepository: Send + Sync {
    async fn list_items(&self) -> anyhow::Result<Vec<Item>>;

    async fn add_item(&self, item: CreateItem) -> anyhow::Result<()>;

    async fn remove_item(&self, item_id: i32, quantity: i32) -> anyhow::Result<()>;

    async fn update_item(&self, item_id: i32, item: UpdateItem) -> anyhow::Result<()>;
}
