use clap::Subcommand;
use rust_decimal::Decimal;

#[derive(Debug, Subcommand)]
pub enum ShoppingCommands {
    List,
    Add {
        #[arg(long)]
        name: String,

        #[arg(long)]
        price: Decimal,

        #[arg(long)]
        quantity: i32,
    },
    Remove {
        item_id: i32,
    },
}
