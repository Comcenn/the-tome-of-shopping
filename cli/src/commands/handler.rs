use shared::{CreateItem, Page, ShoppingListRepository};

use crate::commands::{
    cli::{Cli, Commands},
    pages::{AddItemPage, ListPage},
    shopping::ShoppingCommands,
};

pub async fn handle_command<R: ShoppingListRepository>(
    repo: &R,
    cmd: Cli,
) -> anyhow::Result<Option<Box<dyn Page>>> {
    match cmd.command {
        Commands::Shopping(sub) => match sub {
            ShoppingCommands::List => {
                let items = repo.list_items().await?;
                Ok(Some(Box::new(ListPage::new(items)) as Box<dyn Page>))
            },
            ShoppingCommands::Add { name, price, quantity } => {
                let new_item = CreateItem::new(name, price, quantity);
                repo.add_item(new_item).await?;
                let items = repo.list_items().await?;
                Ok(Some(Box::new(AddItemPage::new(items)) as Box<dyn Page>))
            }

        },
    }
}
