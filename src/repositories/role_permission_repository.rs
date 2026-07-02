use sqlx::PgPool;

use crate::models::role_permission::NewRolePermission;

pub async fn assign_permission(
    pool: &PgPool,
    role_permission: NewRolePermission,
) -> Result<(), sqlx::Error> {

    sqlx::query(
        r#"
        INSERT INTO role_permissions (
            role_id,
            permission_id
        )
        VALUES (
            $1,
            $2
        )
        "#
    )
    .bind(role_permission.role_id)
    .bind(role_permission.permission_id)
    .execute(pool)
    .await?;

    Ok(())
}