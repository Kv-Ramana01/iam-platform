use axum::{extract::{Json, State}, http::{StatusCode, request}};

use crate::{models::user::LoginRequest, state::AppState};
use crate::models::user::RegisterRequest;
use crate::services::auth_service;


use axum::Extension;
use crate::auth::jwt::Claims;

pub async fn root() -> &'static str{
    "Hello from IAM Backend!"
}

pub async fn register(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> StatusCode {
    
    auth_service::register_user(&state.pool,request).await;

    StatusCode::CREATED
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> StatusCode {

    auth_service::login_user(&state.pool,request).await;

    StatusCode::CREATED
}

pub async fn me(
    Extension(claims): Extension<Claims>,
) -> String {
    format!(
        "User ID: {}\nSession ID: {}",
        claims.sub,
        claims.sid,
    )
}