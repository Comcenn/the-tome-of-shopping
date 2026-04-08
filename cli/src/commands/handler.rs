use shared::{
    CreateItem, Page, ShoppingListRepository, email::render_email, item::UpdateItem,
    repository::EmailRepository,
};

use crate::commands::{
    cli::Cli,
    pages::{
        AddItemPage, ListPage, MarkedItemPage, OrderItemPage, RemoveItemPage, SendEmailPage,
        TotalsPage,
    },
    shopping::ShoppingCommands,
};

pub async fn handle_command<R, E>(
    repo: &R,
    email_repo: &E,
    cmd: Cli,
) -> anyhow::Result<Option<Box<dyn Page>>>
where
    R: ShoppingListRepository,
    E: EmailRepository,
{
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
        }
        ShoppingCommands::Reorder { item_id, order } => {
            let updated = UpdateItem::ItemOrder { item_order: order };
            repo.update_item(item_id, updated).await?;
            let items = repo.list_items().await?;
            Ok(Some(Box::new(OrderItemPage::new(items)) as Box<dyn Page>))
        }
        ShoppingCommands::Total { limit } => {
            let items = repo.list_items().await?;
            Ok(Some(
                Box::new(TotalsPage::new(items, limit)) as Box<dyn Page>
            ))
        }
        ShoppingCommands::Email { address } => {
            let items = repo.list_items().await?;
            let message_string = render_email(&items);
            email_repo.send_email(&address, message_string).await?;

            Ok(Some(Box::new(SendEmailPage::new(address)) as Box<dyn Page>))
        }
    }
}
