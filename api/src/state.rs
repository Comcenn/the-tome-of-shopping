use std::sync::Arc;

use shared::ShoppingListRepository;

use crate::config::Config;

#[derive(Clone)]
pub struct AppState<R: ShoppingListRepository> {
    pub config: Config,
    pub repo: Arc<R>,
}

impl<R: ShoppingListRepository> AppState<R> {
    pub fn new(config: Config, repo: Arc<R>) -> Self {
        Self { config, repo }
    }
}
