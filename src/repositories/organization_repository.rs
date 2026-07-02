use sqlx::PgPool;

use crate::models::organization::NewOrganization;






pub async fn create_organization(
    pool: &PgPool,
    organization: NewOrganization,
) -> Result<(), sqlx::Error> {

    sqlx::query(
        r#"
        INSERT INTO organizations (
        id,
        name,
        description)
        VALUES ( $1, $2, $3)
        "#
    ).bind(organization.id)
    .bind(organization.name)
    .bind(organization.description)
    .execute(pool)
    .await?;

    Ok(())
}