use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct Membership {
    pub id: Uuid,
    pub user_id: Uuid,
    pub organization_id: Uuid,
    pub role_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateMembershipRequest {
    pub user_id: Uuid,
    pub organization_id: Uuid,
    pub role_id: Uuid,
}

pub struct NewMembership {
    pub id: Uuid,
    pub user_id: Uuid,
    pub organization_id: Uuid,
    pub role_id: Uuid,
}