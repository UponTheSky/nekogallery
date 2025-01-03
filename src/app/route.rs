use axum::{
    debug_handler,
    extract::{Path, Query},
    Json,
};
use serde_derive::Deserialize;
use uuid::Uuid;

use super::{
    cat::{Cat, CatDraft, CatUpdate},
    error::AppError,
    App,
};

use crate::dependencies::db;

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
            name: String::new(),
        }
    }
}

pub async fn get_all_cats(
    pagination: Option<Query<Pagination>>,
    filter: Option<Query<Filter>>,
    db::Connection(mut conn): db::Connection,
) -> Result<Json<Vec<Cat>>, AppError> {
    // pagination
    let Query(pagination) = pagination.unwrap_or_default();
    let offset = pagination.offset;
    let limit = pagination.limit;

    // filter
    let Query(filter) = filter.unwrap_or_default();
    let name_filter = filter.name;

    let mut query_builder = sqlx::QueryBuilder::new("select id, name from cat");

    if !name_filter.is_empty() {
        query_builder.push(" where name = ").push_bind(name_filter);
    }

    query_builder
        .push(" limit ")
        .push_bind(limit as u32)
        .push(" offset ")
        .push_bind(offset as u32);

    match query_builder
        .build_query_as::<Cat>()
        .fetch_all(&mut *conn)
        .await
    {
        Ok(rows) => Ok(Json(rows)),
        Err(err) => Err(AppError::InternalServerError(format!(
            "internal server error: {}",
            err
        ))),
    }
}

pub async fn get_cat_by_id(
    Path(id): Path<String>,
    db::Connection(mut conn): db::Connection,
) -> Result<Json<Cat>, AppError> {
    let query =
        sqlx::query_as::<sqlx::Sqlite, Cat>("select id, name from cat where id = ?").bind(id);

    match query.fetch_one(&mut *conn).await {
        Ok(cat) => Ok(Json(cat)),
        Err(err) => Err(AppError::NotFound(format!("resource not found: {}", err))),
    }
}

pub async fn post_cat(
    db::Connection(mut conn): db::Connection,
    Json(payload): Json<CatDraft>,
) -> Result<Json<Cat>, AppError> {
    let new_cat = Cat {
        id: Uuid::new_v4().to_string(),
        name: payload.name,
    };

    // update the DB
    let result = sqlx::query("insert into cat (id, name) values (?, ?)")
        .bind(Uuid::new_v4().to_string())
        .bind(new_cat.name.clone())
        .execute(&mut *conn)
        .await;

    match result {
        Ok(_) => Ok(Json(new_cat)),
        Err(err) => Err(AppError::InternalServerError(err.to_string())),
    }
}

pub async fn patch_cat(
    Path(id): Path<String>,
    db::Connection(mut conn): db::Connection,
    Json(payload): Json<CatUpdate>,
) -> Result<Json<Cat>, AppError> {
    let query =
        sqlx::query_as::<sqlx::Sqlite, Cat>("select id, name from cat where id = ?").bind(id);

    match query.fetch_one(&mut *conn).await {
        Ok(cat) => {
            let updated_cat = Cat {
                id: cat.id,
                name: payload.name.unwrap_or(cat.name),
            };

            // update the db
            let result = sqlx::query("update cat set name = ? where id = ?")
                .bind(updated_cat.name.clone())
                .bind(updated_cat.id.clone())
                .execute(&mut *conn)
                .await;

            match result {
                Ok(_) => Ok(Json(updated_cat)),
                Err(err) => Err(AppError::InternalServerError(err.to_string())),
            }
        }
        Err(err) => Err(AppError::NotFound(format!("resource not found: {}", err))),
    }
}
