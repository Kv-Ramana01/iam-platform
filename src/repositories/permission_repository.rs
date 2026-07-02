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

use crate::models::permission::*;

pub async fn get_all_permissions(
    pool: &PgPool,
) -> Result<Vec<Permission>, sqlx::Error> {

    sqlx::query_as::<_, Permission>(
        r#"
        SELECT *
        FROM permissions
        "#
    )
    .fetch_all(pool)
    .await
}