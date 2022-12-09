use axum::{
    routing::{get, post},
    Router,
};

use crate::routes::{
    hello_world::hello_world,
    mirror_body_json::mirror_body_json,
    mirror_body_string::mirror_body_string,
    path_variables::{hard_coded_path, path_variables},
    query_params::query_params,
};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/:id", get(path_variables))
        .route("/path_variables/15", get(hard_coded_path))
        .route("/query_params", get(query_params))
}
