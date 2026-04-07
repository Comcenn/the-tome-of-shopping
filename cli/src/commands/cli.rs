use clap::Parser;

use crate::commands::shopping::ShoppingCommands;

#[derive(Debug, Parser)]
#[command(name = "shopping-cli", version, about = "A shopping list CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: ShoppingCommands,
}
