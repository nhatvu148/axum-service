use axum::{
    routing::{get, post},
    Router,
};

use crate::routes::{
    hello_world::hello_world, mirror_body_json::mirror_body_json,
    mirror_body_string::mirror_body_string,
};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
}
