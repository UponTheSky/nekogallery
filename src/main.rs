use axum::Router;

mod app;

#[tokio::main]
async fn main() {
    let app = app::App::new();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, Router::from(app)).await.unwrap();
}
