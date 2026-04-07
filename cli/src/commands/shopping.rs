use clap::Subcommand;
use rust_decimal::Decimal;

#[derive(Debug, Subcommand)]
pub enum ShoppingCommands {
    List,
    Add {
        name: String,
        price: Decimal,
        quantity: i32
    }
}
