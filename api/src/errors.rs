use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub message: String,
    #[serde(skip)]
    pub status: StatusCode,
}

impl ApiError {
    pub fn unauthorized(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
            status: StatusCode::UNAUTHORIZED,
        }
    }

    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
            status: StatusCode::BAD_REQUEST,
        }
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn from_anyhow(e: anyhow::Error) -> Self {
        ApiError::internal(e.to_string())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let body = Json(serde_json::json!({
            "error": self.message
        }));

        (self.status, body).into_response()
    }
}

impl From<&str> for ApiError {
    fn from(msg: &str) -> Self {
        ApiError::internal(msg)
    }
}

impl From<String> for ApiError {
    fn from(msg: String) -> Self {
        ApiError::internal(msg)
    }
}

pub type Result<T> = std::result::Result<T, ApiError>;
