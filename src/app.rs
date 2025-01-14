use axum::{
    extract::MatchedPath,
    http::Request,
    routing::{get, patch, post, put},
    Router,
};
use sqlx::SqlitePool;
use tower_http::trace::TraceLayer;
use tracing::{info, info_span, Span};

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
            .with_state(db_pool)
            .layer(
                TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!("http_request", method = ?request.method(), matched_path)
                }),
            );

        Self { router }
    }
}

impl From<App> for Router {
    fn from(value: App) -> Self {
        value.router
    }
}
