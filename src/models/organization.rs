
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::Deserialize;



#[derive(sqlx::FromRow)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateOrganizationRequest {
    pub name: String,
    pub description: Option<String>,
}

pub struct NewOrganization {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}