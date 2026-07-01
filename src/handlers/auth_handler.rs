use axum::{extract::{Json, State}, http::StatusCode};

use crate::state::{AppState};
use crate::models::user::RegisterRequest;
use crate::services::auth_service;

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