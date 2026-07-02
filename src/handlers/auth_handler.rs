use axum::{extract::{Json, State}, http::{StatusCode, request}};
use uuid::Uuid;

use crate::{models::{organization::{self, CreateOrganizationRequest}, user::LoginRequest}, state::{self, AppState}};
use crate::models::user::RegisterRequest;
use crate::models::role::*;
use crate::services::auth_service;
use crate::models::permission::CreatePermissionRequest;

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

pub async fn create_organization(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(request): Json<CreateOrganizationRequest>,
) {
    let user_id = Uuid::parse_str(&claims.sub).unwrap();

    auth_service::create_organization(&state.pool, user_id, request).await;
}

pub async fn create_role(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(request): Json<CreateRoleRequest>,
) {
    let user_id = Uuid::parse_str(&claims.sub).unwrap();

    auth_service::create_role(&state.pool, request).await;
}

pub async fn create_permission(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(request): Json<CreatePermissionRequest>,
) {
    auth_service::create_permission(
        &state.pool,
        request,
    )
    .await;
}

use crate::models::role_permission::*;

pub async fn assign_permission(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(request): Json<AssignPermissionRequest>,
) {

    auth_service::assign_permission(
        &state.pool,
        request,
    )
    .await;
}

use crate::models::membership::*;

pub async fn create_membership(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(request): Json<CreateMembershipRequest>,
) {

    auth_service::create_membership(
        &state.pool,
        request,
    )
    .await;
}

//testing purposes
pub async fn check_permission(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) {

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
}