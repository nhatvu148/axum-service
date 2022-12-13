use axum_service::run;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();
    dotenv().ok();
    let database_uri = dotenv!("DATABASE_URL");

    run(database_uri).await
}
