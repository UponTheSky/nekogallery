use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    routing::get,
    Json, Router,
};

use serde::Serialize;

#[tokio::main]
async fn main() {
    let app = make_app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn make_app() -> Router {
    let router = Router::new();

    router.route("/api/cat", get(get_all_cats))
}

use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize)]
struct Cat {
    id: Uuid,
    name: String,
}

impl Default for Cat {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
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

async fn get_all_cats(
    pagination: Option<Query<Pagination>>,
    filter: Option<Query<Filter>>,
) -> Json<Vec<Cat>> {
    let cat1: Cat = Cat {
        id: Uuid::new_v4(),
        name: "miyako".to_string(),
    };

    let cat2: Cat = Cat {
        id: Uuid::new_v4(),
        name: "shibaneko".to_string(),
    };

    let cats = vec![cat1, cat2];

    let Query(pagination) = pagination.unwrap_or_default();
    let offset = pagination.offset;
    let limit = pagination.limit;

    let Query(filter) = filter.unwrap_or_default();
    let name_filter = filter.name;

    let fetched_cats = cats
        .into_iter()
        .skip(offset)
        .take(limit)
        .filter(|cat| cat.name.contains(&name_filter))
        .collect::<Vec<Cat>>();

    Json(fetched_cats)
}
