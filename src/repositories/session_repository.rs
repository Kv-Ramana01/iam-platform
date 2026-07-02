use sqlx::PgPool;
use uuid::Uuid;
use crate::models::session::Session;

pub async fn create_session(
    pool: &PgPool,
    session: &Session,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO sessions (id, user_id,created_at,expires_at,is_active)
        VALUES($1, $2, $3, $4, $5)
        "#,
    ).bind(session.id)
    .bind(session.user_id)
    .bind(session.created_at)
    .bind(session.expires_at)
    .bind(session.is_active)
    .execute(pool).await?;

    Ok(())
}

pub async fn find_session_by_id(
    pool: &PgPool,
    session_id: Uuid,
) -> Result<Option<Session>, sqlx::Error> {
    sqlx::query_as::<_, Session>(
        r#"
        SELECT *
        FROM sessions
        WHERE id = $1
        "#
    ).bind(session_id)
    .fetch_optional(pool)
    .await
}