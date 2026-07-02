use serde::Deserialize;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct RolePermission {
    pub role_id: Uuid,
    pub permission_id: Uuid,
}

#[derive(Deserialize)]
pub struct AssignPermissionRequest {
    pub role_id: Uuid,
    pub permission_id: Uuid,
}

pub struct NewRolePermission {
    pub role_id: Uuid,
    pub permission_id: Uuid,
}