use async_trait::async_trait;
use rust_decimal::dec;
use shared::{Item, ShoppingListRepository};

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::dec;

    #[tokio::test]
    async fn test_list_items_returns_fake_data() {
        let repo = FakeRepo;

        let items = repo.list_items().await.unwrap();

        assert_eq!(items.len(), 3);
        assert_eq!(items[0].name, "Milk");
        assert_eq!(items[0].price, dec!(1.20));
    }
}
