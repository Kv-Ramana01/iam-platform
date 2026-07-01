use crate::models::user::NewUser;
use sqlx::PgPool;


pub async fn create_user (
    pool: &PgPool,
    user: NewUser,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO users (id, name, email, password_hash)
        VALUES ($1, $2, $3, $4)
        "#,
    ).bind(user.id)
    .bind(user.name)
    .bind(user.email)
    .bind(user.password_hash)
    .execute(pool)
    .await?;

    Ok(())
}