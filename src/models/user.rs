use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)] //conerts json to rust struct
pub struct RegisterRequest {
    pub name: String,
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