use rust_decimal::Decimal;

use crate::Item;

pub fn render_email(items: &[Item]) -> String {
    // Sort items by item_order ascending
    let mut items = items.to_vec();
    items.sort_by_key(|i| i.item_order);

    let mut out = String::new();

    out.push_str("Your Shopping List\n");
    out.push_str("====================\n\n");

    out.push_str("Item | Qty | Price | Subtotal\n");
    out.push_str("--------------------------------\n");

    for item in &items {
        let subtotal = (Decimal::from(item.quantity) * item.price).round_dp(2);

        out.push_str(&format!(
            "{} | {} | £{:.2} | £{:.2}\n",
            item.name, item.quantity, item.price, subtotal
        ));
    }

    out.push_str("\nThanks for using the Shopping CLI!\n");

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::{Decimal, dec};

    fn item(id: i32, order: i32, name: &str, price: Decimal, qty: i32) -> Item {
        Item::new(id, order, name, price, qty, false)
    }

    #[test]
    fn render_email_sorts_items_and_formats_subtotals() {
        let items = vec![
            item(1, 20, "Bread", dec!(0.80), 1),
            item(2, 10, "Milk", dec!(1.20), 2),
            item(3, 30, "Eggs", dec!(0.10), 12),
        ];

        let email = render_email(&items);

        // Items should appear sorted by item_order: Milk (10), Bread (20), Eggs (30)
        let milk_pos = email.find("Milk").unwrap();
        let bread_pos = email.find("Bread").unwrap();
        let eggs_pos = email.find("Eggs").unwrap();

        assert!(milk_pos < bread_pos);
        assert!(bread_pos < eggs_pos);

        // Subtotals should be correctly calculated and formatted to 2 decimal places
        assert!(email.contains("Milk | 2 | £1.20 | £2.40"));
        assert!(email.contains("Bread | 1 | £0.80 | £0.80"));
        assert!(email.contains("Eggs | 12 | £0.10 | £1.20"));

        // Header should be present
        assert!(email.contains("Your Shopping List"));
        assert!(email.contains("Item | Qty | Price | Subtotal"));
    }
}
