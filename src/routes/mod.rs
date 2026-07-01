use axum::{
    Router, routing::{get, post},
};

use crate::{handlers::auth_handler, state::AppState};
//get the handler function from handlers which is inside authhander


pub fn create_router() -> Router<AppState> {
    Router::new()
    .route("/",get(auth_handler::root))
    .route("/auth/register", post(auth_handler::register))
    .route(
        "/auth/login", get(auth_handler::login))
}