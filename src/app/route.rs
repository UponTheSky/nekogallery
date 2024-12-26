use axum::{
    debug_handler,
    extract::{rejection::JsonRejection, Path, Query},
    Json,
};
use serde_derive::Deserialize;

use super::{
    cat::{Cat, CatDraft, CatUpdate, CATS},
    error::AppError,
};

#[derive(Deserialize)]
pub struct Pagination {
    pub offset: usize,
    pub limit: usize,
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
pub struct Filter {
    pub name: String,
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            name: String::from(""),
        }
    }
}

pub async fn get_all_cats(
    pagination: Option<Query<Pagination>>,
    filter: Option<Query<Filter>>,
) -> Json<Vec<Cat>> {
    let Query(pagination) = pagination.unwrap_or_default();
    let offset = pagination.offset;
    let limit = pagination.limit;

    let Query(filter) = filter.unwrap_or_default();
    let name_filter = filter.name;

    let fetched_cats = CATS
        .clone()
        .into_iter()
        .skip(offset)
        .take(limit)
        .filter(|cat| cat.name.contains(&name_filter))
        .collect::<Vec<Cat>>();

    Json(fetched_cats)
}

pub async fn get_cat_by_id(Path(id): Path<u32>) -> Result<Json<Cat>, AppError> {
    if let Some(cat) = CATS.clone().into_iter().find(|cat| cat.id == id) {
        Ok(Json(cat))
    } else {
        Err(AppError::NotFound(String::from("resource not found")))
    }
}

#[debug_handler]
pub async fn post_cat(Json(payload): Json<CatDraft>) -> Result<Json<Cat>, AppError> {
    // generate a new entity here
    // TODO: after connecting to DB, change the id to UUID
    let new_cat = Cat {
        id: 2,
        name: payload.name,
    };

    Ok(Json(new_cat))
}

#[debug_handler]
pub async fn patch_cat(
    Path(id): Path<u32>,
    Json(payload): Json<CatUpdate>,
) -> Result<Json<Cat>, AppError> {
    if let Some(cat) = CATS.clone().into_iter().find(|cat| cat.id == id) {
        // TODO: update cat
        let updated_cat = Cat {
            id: cat.id,
            name: payload.name.unwrap_or(cat.name),
        };

        Ok(Json(updated_cat))
    } else {
        Err(AppError::NotFound(String::from("resource not found")))
    }
}
