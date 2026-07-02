use sqlx::PgPool;

use crate::models::membership::NewMembership;

pub async fn create_membership(
    pool: &PgPool,
    membership: NewMembership,
) -> Result<(), sqlx::Error> {

    sqlx::query(
        r#"
        INSERT INTO memberships (
            id,
            user_id,
            organization_id,
            role_id
        )
        VALUES (
            $1,
            $2,
            $3,
            $4
        )
        "#
    )
    .bind(membership.id)
    .bind(membership.user_id)
    .bind(membership.organization_id)
    .bind(membership.role_id)
    .execute(pool)
    .await?;

    Ok(())
}

use uuid::Uuid;

pub async fn has_permission(
    pool: &PgPool,
    user_id: Uuid,
    organization_id: Uuid,
    permission_name: &str,
) -> Result<bool, sqlx::Error> {

    let result = sqlx::query(
        r#"
        SELECT 1
        FROM memberships m
        JOIN role_permissions rp
            ON m.role_id = rp.role_id
        JOIN permissions p
            ON rp.permission_id = p.id
        WHERE
            m.user_id = $1
            AND m.organization_id = $2
            AND p.name = $3
        LIMIT 1
        "#
    )
    .bind(user_id)
    .bind(organization_id)
    .bind(permission_name)
    .fetch_optional(pool)
    .await?;

    Ok(result.is_some())
}