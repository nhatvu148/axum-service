use axum::{routing::get, Router};

use crate::routes::hello_world::hello_world;

pub fn create_router() -> Router {
    Router::new().route("/", get(hello_world))
}
