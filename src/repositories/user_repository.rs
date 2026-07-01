use crate::models::user::{User, NewUser};
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

pub async fn find_user_by_email(
    pool: &PgPool,
    email: &str,
) -> Result<Option<User>, sqlx::Error> {
    //We want sqlx to convert the row into User struct so query_as is used here
    let user = sqlx::query_as::<_,User>(
        r#"
        SELECT *
        FROM users
        WHERE email = $1
        "#
    ).bind(email)
    .fetch_optional(pool).await?;
    //fetch optional basically fetch an option which either contains something or none

    Ok(user)

}