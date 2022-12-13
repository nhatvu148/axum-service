mod database;
mod router;
mod routes;

use std::net::SocketAddr;

use router::create_routes;
use sea_orm::Database;

pub async fn run(database_uri: &str) {
    let database = Database::connect(database_uri).await.unwrap();
    let app = create_routes(database).await;
    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
