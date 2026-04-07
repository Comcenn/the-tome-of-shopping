use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use shared::{CreateItem, Item, ShoppingListRepository};

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

pub async fn add_item_handler<R>(
    State(state): State<AppState<R>>,
    Json(payload): Json<CreateItem>,
) -> Result<StatusCode>
where
    R: ShoppingListRepository + Clone + Send + Sync + 'static,
{
    let controller = ShoppingListController::new(state.repo.clone());
    controller.add_item(payload).await.map_err(|e| ApiError {
        message: e.to_string(),
    })?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn remove_item_handler<R>(
    State(state): State<AppState<R>>,
    Path(item_id): Path<i32>,
) -> Result<StatusCode>
where
    R: ShoppingListRepository + Clone + Send + Sync + 'static,
{
    let controller = ShoppingListController::new(state.repo.clone());
    controller
        .remove_item(item_id)
        .await
        .map_err(|e| ApiError {
            message: e.to_string(),
        })?;
    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use axum::{
        body::{Body, to_bytes},
        http::{Request, StatusCode},
    };
    use std::{sync::Arc, usize};
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

    #[tokio::test]
    async fn add_item_returns_no_content() {
        let repo = Arc::new(FakeRepo);
        let state = AppState::new(Default::default(), repo);
        let app = create_app(state);

        let payload = serde_json::json!({
            "id": 10,
            "category_id": 1,
            "name": "Test Item",
            "price": 1.23,
            "item_order": 1,
            "quantity": 1
        });

        let response = app
            .oneshot(
                Request::post("/shopping/items")
                    .header("content-type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn add_item_rejects_invalid_json() {
        let repo = Arc::new(FakeRepo);
        let state = AppState::new(Default::default(), repo);
        let app = create_app(state);

        // Missing required fields for Item
        let payload = serde_json::json!({
            "name": "OnlyName"
        });

        let response = app
            .oneshot(
                Request::post("/shopping/items")
                    .header("content-type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Axum automatically returns 422 for missing fields JSON -> Item
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn add_item_calls_repository() {
        // Use FakeRepo which always returns Ok(()). We just verify the handler succeeds.
        let repo = Arc::new(FakeRepo);
        let state = AppState::new(Default::default(), repo.clone());
        let app = create_app(state);

        let payload = serde_json::json!({
            "id": 42,
            "category_id": 2,
            "name": "RepoTest",
            "price": 9.99,
            "item_order": 2,
            "quantity": 1
        });

        let response = app
            .oneshot(
                Request::post("/shopping/items")
                    .header("content-type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn remove_item_calls_repository() {
        // Use FakeRepo which always returns Ok(()). We just verify the handler succeeds.
        let repo = Arc::new(FakeRepo);
        let state = AppState::new(Default::default(), repo.clone());
        let app = create_app(state);

        let response = app
            .oneshot(
                Request::delete("/shopping/items/1")
                    .header("content-type", "application/json")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }
}
