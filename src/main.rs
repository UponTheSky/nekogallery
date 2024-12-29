use std::time::Duration;

use axum::Router;

mod app;
mod dependencies;

#[tokio::main]
async fn main() {
    let db_pool = dependencies::db::setup_pool("./sqlite.db", 5, Duration::from_secs(30))
        .await
        .unwrap();

    let app = app::App::new(db_pool);
    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();

    axum::serve(listener, Router::from(app)).await.unwrap();
}
