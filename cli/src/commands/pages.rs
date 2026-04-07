use shared::{Item, Page};

pub struct ListPage {
    pub items: Vec<Item>,
}

impl ListPage {
    pub fn new(items: Vec<Item>) -> Self {
        Self { items }
    }
}

impl Page for ListPage {
    fn render(&self) -> String {
        let mut out = String::new();
        out.push_str("\n========Shopping List========\n");
        out.push_str("Id|Name|Price|Quantity|Order\n");
        for item in &self.items {
            out.push_str(&format!(
                "{}|{}|{}|{}|{}\n",
                item.id, item.name, item.price, item.quantity, item.item_order
            ));
        }
        out.push_str("==========End of List==========\n");

        out
    }
}

pub struct AddItemPage {
    pub items: Vec<Item>,
}

impl AddItemPage {
    pub fn new(items: Vec<Item>) -> Self {
        Self { items }
    }
}

impl Page for AddItemPage {
    fn render(&self) -> String {
        let mut out = String::new();
        out.push_str("\n========Updated Shopping List========\n");
        out.push_str("Id|Name|Price|Quantity|Order\n");
        for item in &self.items {
            out.push_str(&format!(
                "{}|{}|{}|{}|{}\n",
                item.id, item.name, item.price, item.quantity, item.item_order
            ));
        }
        out.push_str("==========End of List==========\n");

        out
    }
}

pub struct RemoveItemPage {
    pub items: Vec<Item>,
}

impl RemoveItemPage {
    pub fn new(items: Vec<Item>) -> Self {
        Self { items }
    }
}

impl Page for RemoveItemPage {
    fn render(&self) -> String {
        let mut out = String::new();
        out.push_str("\n========Updated Shopping List========\n");
        out.push_str("Id|Name|Price|Quantity|Order\n");
        for item in &self.items {
            out.push_str(&format!(
                "{}|{}|{}|{}|{}\n",
                item.id, item.name, item.price, item.quantity, item.item_order
            ));
        }
        out.push_str("==========End of List==========\n");

        out
    }
}
