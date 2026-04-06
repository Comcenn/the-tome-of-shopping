use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Item {
    pub id: i32,
    pub item_order: i32,
    pub name: String,
    pub price: Decimal,
    pub quantity: i32,
}

impl Item {
    pub fn new(
        id: i32,
        item_order: i32,
        name: impl Into<String>,
        price: Decimal,
        quantity: i32,
    ) -> Self {
        Self {
            id,
            item_order,
            name: name.into(),
            price,
            quantity,
        }
    }
}
