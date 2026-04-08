use shared::{CreateItem, Page, ShoppingListRepository, item::UpdateItem};

use crate::commands::{
    cli::Cli,
    pages::{AddItemPage, ListPage, MarkedItemPage, OrderItemPage, RemoveItemPage},
    shopping::ShoppingCommands,
};

pub async fn handle_command<R: ShoppingListRepository>(
    repo: &R,
    cmd: Cli,
) -> anyhow::Result<Option<Box<dyn Page>>> {
    match cmd.command {
        ShoppingCommands::List => {
            let items = repo.list_items().await?;
            Ok(Some(Box::new(ListPage::new(items)) as Box<dyn Page>))
        }
        ShoppingCommands::Add {
            name,
            price,
            quantity,
        } => {
            let new_item = CreateItem::new(name, price, quantity);
            repo.add_item(new_item).await?;
            let items = repo.list_items().await?;
            Ok(Some(Box::new(AddItemPage::new(items)) as Box<dyn Page>))
        }
        ShoppingCommands::Remove { item_id, quantity } => {
            repo.remove_item(item_id, quantity).await?;
            let items = repo.list_items().await?;
            Ok(Some(Box::new(RemoveItemPage::new(items)) as Box<dyn Page>))
        }
        ShoppingCommands::Mark { item_id, ticked } => {
            let updated = UpdateItem::PickedUp { picked_up: ticked };
            repo.update_item(item_id, updated).await?;
            let items = repo.list_items().await?;
            Ok(Some(Box::new(MarkedItemPage::new(items)) as Box<dyn Page>))
        },
        ShoppingCommands::Reorder { item_id, order } => {
            let updated = UpdateItem::ItemOrder { item_order: order };
            repo.update_item(item_id, updated).await?;
            let items = repo.list_items().await?;
            Ok(Some(Box::new(OrderItemPage::new(items)) as Box<dyn Page>))
        }
    }
}
