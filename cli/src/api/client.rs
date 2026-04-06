use std::time::Duration;

use async_trait::async_trait;
use reqwest::Client;
use shared::{Item, ShoppingListRepository};
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
}

#[cfg(test)]
mod tests {
    use httpmock::{Method::GET, MockServer};
    use rust_decimal::dec;

    use super::*;

    #[tokio::test]
    async fn test_list_items_success() {
        let server = MockServer::start();

        let items = vec![
            Item::new(1, 1, "Milk", dec!(2.50), 1),
            Item::new(2, 2, "Bread", dec!(4.99), 1),
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
}
