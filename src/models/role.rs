
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow)]
pub struct Role {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateRoleRequest {
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

pub struct NewRole {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,
}