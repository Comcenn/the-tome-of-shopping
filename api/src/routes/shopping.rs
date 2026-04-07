use axum::{
    Router,
    routing::{get, post},
};
use shared::ShoppingListRepository;
use state::AppState;

use crate::{
    handlers::shopping::{add_item_handler, list_items_handler},
    state,
};

pub fn routes<R>(state: AppState<R>) -> Router
where
    R: ShoppingListRepository + Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(list_items_handler::<R>))
        .route("/items", post(add_item_handler::<R>))
        .with_state(state)
}
