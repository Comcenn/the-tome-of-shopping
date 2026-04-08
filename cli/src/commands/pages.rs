use shared::{Item, Page};

/// Shared helper to render any page of items.
/// Sorts items by `item_order` (ascending) before printing.
fn render_items(header: &str, items: &[Item]) -> String {
    let mut items = items.to_vec();
    items.sort_by_key(|i| i.item_order);

    let mut out = String::new();
    out.push_str(header);
    out.push('\n');
    out.push_str("Id|Name|Price|Quantity|Order|Picked Up\n");

    for item in &items {
        out.push_str(&format!(
            "{}|{}|{}|{}|{}|{}\n",
            item.id,
            item.name,
            item.price,
            item.quantity,
            item.item_order,
            item.picked_up
        ));
    }

    out.push_str("==========End of List==========\n");
    out
}

//
// LIST PAGE
//

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
        render_items("\n========Shopping List========", &self.items)
    }
}

//
// ADD ITEM PAGE
//

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
        render_items("\n========Updated Shopping List========", &self.items)
    }
}

//
// REMOVE ITEM PAGE
//

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
        render_items("\n========Updated Shopping List========", &self.items)
    }
}

//
// MARKED ITEM PAGE
//

pub struct MarkedItemPage {
    pub items: Vec<Item>,
}

impl MarkedItemPage {
    pub fn new(items: Vec<Item>) -> Self {
        Self { items }
    }
}

impl Page for MarkedItemPage {
    fn render(&self) -> String {
        render_items("\n========Updated Shopping List========", &self.items)
    }
}

//
// ORDER ITEM PAGE
//

pub struct OrderItemPage {
    pub items: Vec<Item>,
}

impl OrderItemPage {
    pub fn new(items: Vec<Item>) -> Self {
        Self { items }
    }
}

impl Page for OrderItemPage {
    fn render(&self) -> String {
        render_items("\n========Updated Shopping List========", &self.items)
    }
}
