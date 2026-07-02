use sqlx::PgPool;
use crate::models::role::*;
use uuid::Uuid;

pub async fn create_role(
    pool: &PgPool,
    role: NewRole,
) -> Result<Uuid, sqlx::Error> {
    sqlx::query(
        r#"
            INSERT into roles (
                id,
                organization_id,
                name,
                description
            )
            VALUES(
                $1,
                $2,
                $3,
                $4
            )
        "#
    ).bind(role.id)
.bind(role.organization_id)
.bind(role.name)
.bind(role.description)
.execute(pool)
.await?;

Ok(role.id)
}