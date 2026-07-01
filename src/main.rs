mod handlers;
mod routes;
mod services;
mod config;

//dependencies : uuid for assigning globally unique ids to users
//chrono to handle dates like created_at, updated_at, expires_at

#[tokio::main] //attribute
async fn main() {
    let app = routes::create_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    let pool = config::database::create_pool().await;

    println!("database connected successfuly.");

    axum::serve(listener, app).await.unwrap();

}
