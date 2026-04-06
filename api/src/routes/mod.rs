use std::time::Duration;

use crate::state::AppState;
use axum::{Router, http::StatusCode};
use shared::ShoppingListRepository;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};

pub mod shopping;

pub fn create_routes<R>(state: AppState<R>) -> Router
where
    R: ShoppingListRepository + Clone + Send + Sync + 'static,
{
    Router::new()
        .nest("/shopping", shopping::routes(state))
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(10)),
        ))
}
