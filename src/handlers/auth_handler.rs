use axum::{
    extract::{Json, State},
    http::StatusCode,
    Extension,
};
use uuid::Uuid;

use crate::{
    auth::jwt::Claims,
    models::{
        membership::*,
        organization::CreateOrganizationRequest,
        permission::CreatePermissionRequest,
        role::*,
        role_permission::*,
        user::{LoginRequest, RegisterRequest},
    },
    services::auth_service,
    state::AppState,
};

pub async fn root() -> &'static str {
    "Hello from IAM Backend!"
}

pub async fn register(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> StatusCode {

    auth_service::register_user(&state.pool, request).await
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> StatusCode {

    auth_service::login_user(&state.pool, request).await
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

pub async fn create_organization(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(request): Json<CreateOrganizationRequest>,
) -> StatusCode {

    let user_id = Uuid::parse_str(&claims.sub).unwrap();

    auth_service::create_organization(
        &state.pool,
        user_id,
        request,
    )
    .await
}

pub async fn create_role(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(request): Json<CreateRoleRequest>,
) -> StatusCode {

    let user_id = Uuid::parse_str(&claims.sub).unwrap();

    auth_service::create_role(
        &state.pool,
        user_id,
        request,
    )
    .await
}

pub async fn create_permission(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(request): Json<CreatePermissionRequest>,
) -> StatusCode {

    auth_service::create_permission(
        &state.pool,
        request,
    )
    .await
}

pub async fn assign_permission(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(request): Json<AssignPermissionRequest>,
) -> StatusCode {

    auth_service::assign_permission(
        &state.pool,
        request,
    )
    .await
}

pub async fn create_membership(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(request): Json<CreateMembershipRequest>,
) -> StatusCode {

    auth_service::create_membership(
        &state.pool,
        request,
    )
    .await
}

// Temporary testing endpoint
pub async fn check_permission(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> StatusCode {

    let user_id = Uuid::parse_str(&claims.sub).unwrap();

    let organization_id =
        Uuid::parse_str("8835bfa7-8b56-472d-94f3-bf9208b9159a").unwrap();

    let allowed = auth_service::has_permission(
        &state.pool,
        user_id,
        organization_id,
        "create_role",
    )
    .await;

    println!("Allowed: {}", allowed);

    StatusCode::OK
}