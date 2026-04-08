use async_trait::async_trait;

use crate::{
    CreateItem,
    item::{Item, UpdateItem},
    user::UserId,
};

#[async_trait]
pub trait ShoppingListRepository: Send + Sync {
    async fn list_items(&self, user: UserId, password: &str) -> anyhow::Result<Vec<Item>>;

    async fn add_item(&self, item: CreateItem, user: UserId, password: &str) -> anyhow::Result<()>;

    async fn remove_item(
        &self,
        item_id: i32,
        quantity: i32,
        user: UserId,
        password: &str,
    ) -> anyhow::Result<()>;

    async fn update_item(
        &self,
        item_id: i32,
        item: UpdateItem,
        user: UserId,
        password: &str,
    ) -> anyhow::Result<()>;
}

#[async_trait]
pub trait EmailRepository: Send + Sync {
    async fn send_email(&self, email: &str, message: String) -> anyhow::Result<()>;
}
