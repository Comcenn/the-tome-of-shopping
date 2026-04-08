use axum::http::{HeaderMap, header};
use base64::Engine;
use shared::user::{UserContext, UserId};

use crate::errors::ApiError;

pub fn user_context_from_headers(headers: &HeaderMap) -> Result<UserContext, ApiError> {
    // 1. Extract Authorization header
    let auth_header = headers
        .get(header::AUTHORIZATION)
        .ok_or_else(|| ApiError::unauthorized("Missing Authorization header"))?
        .to_str()
        .map_err(|_| ApiError::unauthorized("Invalid Authorization header"))?;

    // 2. Must start with "Basic "
    let encoded = auth_header
        .strip_prefix("Basic ")
        .ok_or_else(|| ApiError::unauthorized("Authorization must use Basic scheme"))?;

    // 3. Decode Base64
    let decoded_bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|_| ApiError::unauthorized("Invalid Base64 in Authorization header"))?;

    // 4. Convert to UTF‑8
    let decoded = std::str::from_utf8(&decoded_bytes)
        .map_err(|_| ApiError::unauthorized("Authorization header is not valid UTF‑8"))?;

    // 5. Split "username:password"
    let mut parts = decoded.splitn(2, ':');

    let username = parts
        .next()
        .filter(|s: &&str| !s.is_empty()) // <-- explicit type fixes your error
        .ok_or_else(|| ApiError::unauthorized("Missing username"))?;

    let password = parts
        .next()
        .filter(|s: &&str| !s.is_empty()) // <-- same here
        .ok_or_else(|| ApiError::unauthorized("Missing password"))?;

    Ok(UserContext::new(
        UserId(username.to_string()),
        password.to_string(),
    ))
}
