use axum::{
    middleware,Router, routing::{get, post},
};

use crate::{
    middleware::auth_middleware,
    state::AppState,
};

use crate::{handlers::auth_handler};
//get the handler function from handlers which is inside authhander


pub fn create_router(
    state: AppState,
) -> Router {
    let protected_routes = Router::new().route("/users/me", get(auth_handler::me))
    .route_layer(
        middleware::from_fn_with_state(
            state.clone(), auth_middleware::auth),
    );
    Router::new()
    .route("/",get(auth_handler::root))
    .route("/auth/register", post(auth_handler::register))
    .route(
        "/auth/login", post(auth_handler::login))
        .merge(protected_routes)
        .with_state(state)
}