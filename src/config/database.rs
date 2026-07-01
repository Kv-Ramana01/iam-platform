use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;


//PgPool -< stores multiple postgresql connections
//using it to create a pool of connections
pub async fn create_pool() -> PgPool {
    dotenvy::dotenv().ok(); //loads the env file

    let database_url = 
        env::var("DATABASE_URL").expect("DATABASE_URL must be set"); //gets the db url from memory

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database....")
}