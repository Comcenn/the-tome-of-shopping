use axum::{Json, extract::State};
use shared::{Item, ShoppingListRepository};

use crate::{
    controllers::shopping_list::ShoppingListController,
    errors::{ApiError, Result},
    state::AppState,
};

pub async fn list_items_handler<R>(State(state): State<AppState<R>>) -> Result<Json<Vec<Item>>>
where
    R: ShoppingListRepository + Clone + Send + Sync + 'static,
{
    let controller = ShoppingListController::new(state.repo.clone());
    let items = controller.list_items().await.map_err(|e| ApiError {
        message: e.to_string(),
    })?;
    Ok(Json(items))
}

#[cfg(test)]
mod tests {
    use axum::{
        body::{Body, to_bytes},
        http::{Request, StatusCode},
    };
    use std::sync::Arc;
    use tower::ServiceExt;

    use crate::{create_app, repositories::FakeRepo, state::AppState};

    #[tokio::test]
    async fn list_items_returns_ok() {
        let repo = Arc::new(FakeRepo);
        let state = AppState::new(Default::default(), repo);
        let app = create_app(state);

        let response = app
            .oneshot(Request::get("/shopping").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn list_items_returns_expected_json() {
        let repo = Arc::new(FakeRepo);
        let state = AppState::new(Default::default(), repo);
        let app = create_app(state);

        let response = app
            .oneshot(Request::get("/shopping").body(Body::empty()).unwrap())
            .await
            .unwrap();

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let items: Vec<shared::Item> = serde_json::from_slice(&body).unwrap();

        assert_eq!(items.len(), 3);
        assert_eq!(items[0].name, "Milk");
    }
}
