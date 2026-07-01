mod handlers;
mod routes;
mod services;
mod config;
mod models;
mod repositories;

mod state;
//dependencies : uuid for assigning globally unique ids to users
//chrono to handle dates like created_at, updated_at, expires_at

#[tokio::main] //attribute
async fn main() {
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    
    let pool = config::database::create_pool().await;
    println!("database connected successfuly.");
    
    let state = state::AppState {
        pool,
    };
    
    let app = routes::create_router().with_state(state);

    axum::serve(listener, app).await.unwrap();

}
