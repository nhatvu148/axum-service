use axum::{
    extract::FromRef,
    http::Method,
    middleware,
    routing::{get, post},
    Extension, Router,
};
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};

use crate::routes::{
    always_errors::always_errors,
    create_task::create_task,
    custom_json_extractor::custom_json_extractor,
    get_json::get_json,
    get_tasks::{get_all_tasks, get_one_task},
    hello_world::hello_world,
    middleware_message::middleware_message,
    mirror_body_json::mirror_body_json,
    mirror_body_string::mirror_body_string,
    mirror_custom_header::mirror_custom_header,
    mirror_user_agent::mirror_user_agent,
    path_variables::{hard_coded_path, path_variables},
    query_params::query_params,
    read_middleware_custom_header::read_middleware_custom_header,
    returns_201::returns_201,
    set_middleware_custom_header::set_middleware_custom_header,
    validate_with_serde::validate_with_serde,
};

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

#[derive(Clone, FromRef)]
pub struct AppState {
    pub database: DatabaseConnection,
}

pub async fn create_routes(database: DatabaseConnection) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);
    let shared_data = SharedData {
        message: "Hello from shared data".to_owned(),
    };

    let app_state = AppState { database };

    Router::new()
        .route(
            "/read_middleware_custom_header",
            get(read_middleware_custom_header),
        )
        .route_layer(middleware::from_fn(set_middleware_custom_header))
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/:id", get(path_variables))
        .route("/path_variables/15", get(hard_coded_path))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .route("/always_errors", get(always_errors))
        .route("/returns_201", post(returns_201))
        .route("/get_json", get(get_json))
        .route("/validate_with_serde", post(validate_with_serde))
        .route("/custom_json_extractor", post(custom_json_extractor))
        .route("/tasks", post(create_task))
        .route("/tasks", get(get_all_tasks))
        .route("/tasks/:task_id", get(get_one_task))
        .layer(Extension(shared_data))
        .with_state(app_state)
        .layer(cors)
}
