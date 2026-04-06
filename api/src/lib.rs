use axum::Router;

use shared::ShoppingListRepository;
use state::AppState;

pub mod config;
pub mod controllers;
pub mod errors;
pub mod handlers;
pub mod repositories;
pub mod routes;
pub mod state;

pub fn create_app<R>(state: AppState<R>) -> Router
where
    R: ShoppingListRepository + Clone + Send + Sync + 'static,
{
    routes::create_routes(state)
}
