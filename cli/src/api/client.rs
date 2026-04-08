use std::time::Duration;

use async_trait::async_trait;
use reqwest::Client;
use shared::{
    CreateItem, Item, ShoppingListRepository,
    item::{RemoveItem, UpdateItem},
};
use url::Url;

pub struct ShoppingListClient {
    web_client: Client,
    base_url: Url,
}

impl ShoppingListClient {
    pub fn new(web_client: Client, base_url: Url) -> Self {
        Self {
            web_client: web_client.to_owned(),
            base_url,
        }
    }

    pub fn build(host: &str) -> anyhow::Result<Self> {
        let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
        let base_url = Url::parse(host)?;
        Ok(ShoppingListClient::new(client, base_url))
    }
}

#[async_trait]
impl ShoppingListRepository for ShoppingListClient {
    async fn list_items(&self) -> anyhow::Result<Vec<Item>> {
        let url = self.base_url.join("shopping")?;

        let items = self
            .web_client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<Item>>()
            .await?;

        Ok(items)
    }

    async fn add_item(&self, item: CreateItem) -> anyhow::Result<()> {
        let url = self.base_url.join("shopping/items")?;

        self.web_client
            .post(url)
            .json(&item)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    async fn remove_item(&self, item_id: i32, quantity: i32) -> anyhow::Result<()> {
        let url = self.base_url.join(&format!("shopping/items/{}", item_id))?;
        let payload = RemoveItem::new(quantity);

        self.web_client
            .delete(url)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    async fn update_item(&self, item_id: i32, item: UpdateItem) -> anyhow::Result<()> {
        let url = self.base_url.join(&format!("shopping/items/{}", item_id))?;

        self.web_client
            .patch(url)
            .json(&item)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use httpmock::{
        Method::{DELETE, GET, PATCH, POST},
        MockServer,
    };
    use rust_decimal::dec;

    use super::*;

    #[tokio::test]
    async fn test_list_items_success() {
        let server = MockServer::start();

        let items = vec![
            Item::new(1, 1, "Milk", dec!(2.50), 1, false),
            Item::new(2, 2, "Bread", dec!(4.99), 1, false),
        ];

        let mock = server.mock(|when, then| {
            when.method(GET).path("/shopping");
            then.status(200).json_body_obj(&items);
        });

        let client = ShoppingListClient::build(&server.base_url()).unwrap();

        let result = client.list_items().await.unwrap();

        assert_eq!(result, items);

        mock.assert();
    }

    #[tokio::test]
    async fn test_list_items_returns_error_on_404() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET).path("/shopping");
            then.status(404);
        });

        let client = ShoppingListClient::build(&server.base_url()).unwrap();

        let result = client.list_items().await;

        assert!(result.is_err());

        mock.assert();
    }

    #[tokio::test]
    async fn test_add_item_success() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(POST).path("/shopping/items");
            then.status(204);
        });

        let test_item = CreateItem::new("item", dec!(10.99), 1);

        let client = ShoppingListClient::build(&server.base_url()).unwrap();

        let result = client.add_item(test_item).await;

        assert!(result.is_ok());

        mock.assert();
    }

    #[tokio::test]
    async fn test_add_item_returns_err_on_500() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(POST).path("/shopping/items");
            then.status(500);
        });

        let test_item = CreateItem::new("item", dec!(10.99), 1);

        let client = ShoppingListClient::build(&server.base_url()).unwrap();

        let result = client.add_item(test_item).await;

        assert!(result.is_err());

        mock.assert();
    }

    #[tokio::test]
    async fn test_remove_item_success() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(DELETE).path("/shopping/items/11");
            then.status(204);
        });

        let client = ShoppingListClient::build(&server.base_url()).unwrap();

        let result = client.remove_item(11, 1).await;

        assert!(result.is_ok());

        mock.assert();
    }

    #[tokio::test]
    async fn test_remove_item_returns_err_on_500() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(DELETE).path("/shopping/items/11");
            then.status(500);
        });

        let client = ShoppingListClient::build(&server.base_url()).unwrap();

        let result = client.remove_item(11, 1).await;

        assert!(result.is_err());

        mock.assert();
    }

    #[tokio::test]
    async fn test_update_item_success() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(PATCH).path("/shopping/items/11");
            then.status(204);
        });

        let update_item = UpdateItem::PickedUp { picked_up: true };

        let client = ShoppingListClient::build(&server.base_url()).unwrap();

        let result = client.update_item(11, update_item).await;

        assert!(result.is_ok());

        mock.assert();
    }

    #[tokio::test]
    async fn test_update_item_returns_err_on_500() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(PATCH).path("/shopping/items/11");
            then.status(500);
        });

        let update_item = UpdateItem::PickedUp { picked_up: true };

        let client = ShoppingListClient::build(&server.base_url()).unwrap();

        let result = client.update_item(11, update_item).await;

        assert!(result.is_err());

        mock.assert();
    }
}
