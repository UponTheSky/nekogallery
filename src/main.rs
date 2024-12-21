use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};

#[tokio::main]
async fn main() {
    let app = make_app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn make_app() -> Router {
    let router = Router::new();

    router
        .route("/api/cat", get(get_all_cats))
        .route("/api/cat/:id", get(get_cat_by_id))
}

use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
struct Cat {
    id: u32,
    name: String,
}

impl Default for Cat {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::from("noname"),
        }
    }
}

#[derive(Deserialize)]
struct Pagination {
    offset: usize,
    limit: usize,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: 6,
        }
    }
}

#[derive(Deserialize)]
struct Filter {
    name: String,
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            name: String::from(""),
        }
    }
}

enum AppError {
    NOT_FOUND,
    INTERNAL_SERVER_ERROR,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, message) = match self {
            AppError::NOT_FOUND => (StatusCode::NOT_FOUND, "resource not found"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "internal server error"),
        };

        (status_code, message).into_response()
    }
}

static cats: std::sync::LazyLock<Vec<Cat>> = std::sync::LazyLock::new(|| {
    vec![
        Cat {
            id: 0,
            name: "miyako".to_string(),
        },
        Cat {
            id: 1,
            name: "shibaneko".to_string(),
        },
    ]
});

async fn get_all_cats(
    pagination: Option<Query<Pagination>>,
    filter: Option<Query<Filter>>,
) -> Json<Vec<Cat>> {
    let Query(pagination) = pagination.unwrap_or_default();
    let offset = pagination.offset;
    let limit = pagination.limit;

    let Query(filter) = filter.unwrap_or_default();
    let name_filter = filter.name;

    let fetched_cats = cats
        .clone()
        .into_iter()
        .skip(offset)
        .take(limit)
        .filter(|cat| cat.name.contains(&name_filter))
        .collect::<Vec<Cat>>();

    Json(fetched_cats)
}

async fn get_cat_by_id(Path(id): Path<u32>) -> Result<Json<Cat>, AppError> {
    if let Some(cat) = cats.clone().into_iter().find(|cat| cat.id == id) {
        Ok(Json(cat))
    } else {
        Err(AppError::NOT_FOUND)
    }
}
