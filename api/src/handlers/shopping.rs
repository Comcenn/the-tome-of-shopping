use crate::{
    controllers::shopping_list::ShoppingListController,
    errors::{ApiError, Result},
    extractors::user_context_from_headers, // <-- your new strict extractor
    state::AppState,
};
use axum::{
    Json,
    extract::{Path, State},
    http::HeaderMap,
    http::StatusCode,
};
use shared::{
    CreateItem, Item, ShoppingListRepository,
    item::{RemoveItem, UpdateItem},
};

pub async fn list_items_handler<R>(
    State(state): State<AppState<R>>,
    headers: HeaderMap,
) -> Result<Json<Vec<Item>>>
where
    R: ShoppingListRepository + Clone + Send + Sync + 'static,
{
    let ctx = user_context_from_headers(&headers)?; // <-- now returns Result
    let controller = ShoppingListController::new(state.repo.clone());

    let items = controller
        .list_items(ctx.user.clone(), &ctx.password)
        .await
        .map_err(ApiError::from_anyhow)?;

    Ok(Json(items))
}

pub async fn add_item_handler<R>(
    State(state): State<AppState<R>>,
    headers: HeaderMap,
    Json(payload): Json<CreateItem>,
) -> Result<StatusCode>
where
    R: ShoppingListRepository + Clone + Send + Sync + 'static,
{
    let ctx = user_context_from_headers(&headers)?;
    let controller = ShoppingListController::new(state.repo.clone());

    controller
        .add_item(payload, ctx.user.clone(), &ctx.password)
        .await
        .map_err(ApiError::from_anyhow)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn remove_item_handler<R>(
    State(state): State<AppState<R>>,
    headers: HeaderMap,
    Path(item_id): Path<i32>,
    Json(payload): Json<RemoveItem>,
) -> Result<StatusCode>
where
    R: ShoppingListRepository + Clone + Send + Sync + 'static,
{
    let ctx = user_context_from_headers(&headers)?;
    let controller = ShoppingListController::new(state.repo.clone());

    controller
        .remove_item(item_id, payload.quantity, ctx.user.clone(), &ctx.password)
        .await
        .map_err(ApiError::from_anyhow)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_item_handler<R>(
    State(state): State<AppState<R>>,
    headers: HeaderMap,
    Path(item_id): Path<i32>,
    Json(payload): Json<UpdateItem>,
) -> Result<StatusCode>
where
    R: ShoppingListRepository + Clone + Send + Sync + 'static,
{
    let ctx = user_context_from_headers(&headers)?;
    let controller = ShoppingListController::new(state.repo.clone());

    controller
        .update_item(item_id, payload, ctx.user.clone(), &ctx.password)
        .await
        .map_err(ApiError::from_anyhow)?;

    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::{Body, to_bytes},
        http::{HeaderValue, Request, StatusCode},
    };
    use base64::Engine;
    use rust_decimal::dec;
    use shared::{CreateItem, InMemoryRepo, ShoppingListRepository};
    use std::sync::Arc;
    use tower::ServiceExt;

    use crate::{create_app, state::AppState};

    fn basic_auth(username: &str, password: &str) -> HeaderValue {
        let encoded =
            base64::engine::general_purpose::STANDARD.encode(format!("{username}:{password}"));
        HeaderValue::from_str(&format!("Basic {encoded}")).unwrap()
    }

    fn sample_item(name: &str) -> CreateItem {
        CreateItem {
            name: name.into(),
            price: dec!(1.0),
            quantity: 1,
        }
    }

    // -----------------------------
    // LIST ITEMS
    // -----------------------------

    #[tokio::test]
    async fn list_items_requires_auth() {
        let repo = Arc::new(InMemoryRepo::new());
        let state = AppState::new(Default::default(), repo);
        let app = create_app(state);

        let response = app
            .oneshot(Request::get("/shopping").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn list_items_returns_items_for_valid_auth() {
        let repo = Arc::new(InMemoryRepo::new());

        repo.add_item(sample_item("Milk"), "alice".into(), "pw")
            .await
            .unwrap();
        repo.add_item(sample_item("Bread"), "alice".into(), "pw")
            .await
            .unwrap();

        let state = AppState::new(Default::default(), repo);
        let app = create_app(state);

        let response = app
            .oneshot(
                Request::get("/shopping")
                    .header("Authorization", basic_auth("alice", "pw"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let items: Vec<Item> = serde_json::from_slice(&body).unwrap();

        assert_eq!(items.len(), 2);
        assert_eq!(items[0].name, "Milk");
        assert_eq!(items[1].name, "Bread");
    }

    // -----------------------------
    // ADD ITEM
    // -----------------------------

    #[tokio::test]
    async fn add_item_requires_auth() {
        let repo = Arc::new(InMemoryRepo::new());
        let state = AppState::new(Default::default(), repo);
        let app = create_app(state);

        let payload = serde_json::json!({
            "name": "Test",
            "price": 1.0,
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

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn add_item_accepts_valid_auth() {
        let repo = Arc::new(InMemoryRepo::new());
        let state = AppState::new(Default::default(), repo.clone());
        let app = create_app(state);

        let payload = serde_json::json!({
            "name": "Test",
            "price": 1.0,
            "quantity": 1
        });

        let response = app
            .oneshot(
                Request::post("/shopping/items")
                    .header("Authorization", basic_auth("bob", "pw"))
                    .header("content-type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);

        let items = repo.list_items("bob".into(), "pw").await.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "Test");
    }

    #[tokio::test]
    async fn add_item_rejects_invalid_json() {
        let repo = Arc::new(InMemoryRepo::new());
        let state = AppState::new(Default::default(), repo);
        let app = create_app(state);

        let payload = serde_json::json!({
            "name": "OnlyName"
        });

        let response = app
            .oneshot(
                Request::post("/shopping/items")
                    .header("Authorization", basic_auth("bob", "pw"))
                    .header("content-type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    // -----------------------------
    // REMOVE ITEM
    // -----------------------------

    #[tokio::test]
    async fn remove_item_requires_auth() {
        let repo = Arc::new(InMemoryRepo::new());
        let state = AppState::new(Default::default(), repo);
        let app = create_app(state);

        let payload = serde_json::json!({ "quantity": 1 });

        let response = app
            .oneshot(
                Request::delete("/shopping/items/1")
                    .header("content-type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn remove_item_calls_repo() {
        let repo = Arc::new(InMemoryRepo::new());
        let state = AppState::new(Default::default(), repo.clone());
        let app = create_app(state);

        repo.add_item(sample_item("Milk"), "dave".into(), "pw")
            .await
            .unwrap();

        let payload = serde_json::json!({ "quantity": 1 });

        let response = app
            .oneshot(
                Request::delete("/shopping/items/1")
                    .header("Authorization", basic_auth("dave", "pw"))
                    .header("content-type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);

        let items = repo.list_items("dave".into(), "pw").await.unwrap();
        assert!(items.is_empty());
    }

    // -----------------------------
    // UPDATE ITEM
    // -----------------------------

    #[tokio::test]
    async fn update_item_requires_auth() {
        let repo = Arc::new(InMemoryRepo::new());
        let state = AppState::new(Default::default(), repo);
        let app = create_app(state);

        let payload = serde_json::json!({
            "type": "picked_up",
            "picked_up": true
        });

        let response = app
            .oneshot(
                Request::patch("/shopping/items/1")
                    .header("content-type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn update_item_calls_repo() {
        let repo = Arc::new(InMemoryRepo::new());
        let state = AppState::new(Default::default(), repo.clone());
        let app = create_app(state);

        repo.add_item(sample_item("Eggs"), "carol".into(), "pw")
            .await
            .unwrap();

        let payload = serde_json::json!({
            "type": "picked_up",
            "picked_up": true
        });

        let response = app
            .oneshot(
                Request::patch("/shopping/items/1")
                    .header("Authorization", basic_auth("carol", "pw"))
                    .header("content-type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);

        let items = repo.list_items("carol".into(), "pw").await.unwrap();
        assert!(items[0].picked_up);
    }
}
