use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Item {
    pub id: i32,
    pub item_order: i32,
    pub name: String,
    pub price: Decimal,
    pub quantity: i32,
    pub picked_up: bool,
}

impl Item {
    pub fn new(
        id: i32,
        item_order: i32,
        name: impl Into<String>,
        price: Decimal,
        quantity: i32,
        picked_up: bool,
    ) -> Self {
        Self {
            id,
            item_order,
            name: name.into(),
            price,
            quantity,
            picked_up,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CreateItem {
    pub name: String,
    pub price: Decimal,
    pub quantity: i32,
}

impl CreateItem {
    pub fn new(name: impl Into<String>, price: Decimal, quantity: i32) -> Self {
        Self {
            name: name.into(),
            price,
            quantity,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct RemoveItem {
    pub quantity: i32,
}

impl RemoveItem {
    pub fn new(quantity: i32) -> Self {
        Self { quantity }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum UpdateItem {
    PickedUp { picked_up: bool },
    ItemOrder { item_order: i32 },
}
