use shared::{Page, ShoppingListRepository};

use crate::commands::{cli::{Cli, Commands}, pages::ListPage, shopping::ShoppingCommands};

pub async fn handle_command<R: ShoppingListRepository>(repo: &R, cmd: Cli) -> anyhow::Result<Option<Box<dyn Page>>> {
    match cmd.command {
        Commands::Shopping(sub) => match sub {
            ShoppingCommands::List => {
                let items = repo.list_items().await?;
                Ok(Some(Box::new(ListPage::new(items))))
            }
        }
    }
}