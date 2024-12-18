use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = make_app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn make_app() -> Router {
    let router = Router::new();

    router.route("/:string", get(echo))
}

async fn echo(Path(string): Path<String>) -> String {
    println!("get the word {}", &string);
    string
}
