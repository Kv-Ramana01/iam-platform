use sqlx::PgPool;

use crate::models::permission::NewPermission;

pub async fn create_permission(
    pool: &PgPool,
    permission: NewPermission,
) -> Result<(), sqlx::Error> {

    sqlx::query(
        r#"
        INSERT INTO permissions (
            id,
            name,
            description
        )
        VALUES (
            $1,
            $2,
            $3
        )
        "#
    )
    .bind(permission.id)
    .bind(permission.name)
    .bind(permission.description)
    .execute(pool)
    .await?;

    Ok(())
}