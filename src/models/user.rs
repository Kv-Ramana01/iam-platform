use serde::Deserialize;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Deserialize)] //conerts json to rust struct
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub struct NewUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}
//to hash passwords we will use argon2

#[derive(sqlx::FromRow)]//converts the retrieved row info to rust struct
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}