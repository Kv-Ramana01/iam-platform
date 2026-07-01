use axum::{
    routing::get,
    Router,
};

use crate::handlers::auth_handler;
//get the handler function from handlers which is inside authhander

pub fn create_router() -> Router {
    Router::new().route("/", get(auth_handler::root))
}