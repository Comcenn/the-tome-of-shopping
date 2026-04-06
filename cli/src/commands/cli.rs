use clap::{Parser, Subcommand};

use crate::commands::shopping::ShoppingCommands;

#[derive(Debug, Parser)]
#[command(name = "shopping-cli", version, about = "A shopping list CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    Shopping(ShoppingCommands),
}
