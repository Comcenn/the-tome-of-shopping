use axum::{Router, routing::get};
use shared::ShoppingListRepository;
use state::AppState;

use crate::{handlers::shopping::list_items_handler, state};

pub fn routes<R>(state: AppState<R>) -> Router
where
    R: ShoppingListRepository + Clone + Send + Sync + 'static,
{
    Router::new().route("/", get(list_items_handler::<R>).with_state(state))
}
