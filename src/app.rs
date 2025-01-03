use axum::{
    routing::{get, patch, post, put},
    Router,
};
use sqlx::SqlitePool;

mod cat;
mod error;
mod route;

pub struct App {
    router: Router,
}

impl App {
    pub fn new(db_pool: SqlitePool) -> Self {
        let router = Router::new()
            .route("/api/cat", get(route::get_all_cats))
            .route("/api/cat/:id", get(route::get_cat_by_id))
            .route("/api/cat", post(route::post_cat))
            .route("/api/cat/:id", patch(route::patch_cat))
            .with_state(db_pool);

        Self { router }
    }
}

impl From<App> for Router {
    fn from(value: App) -> Self {
        value.router
    }
}
