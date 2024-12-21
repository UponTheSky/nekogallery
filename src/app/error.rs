use axum::{http::StatusCode, response::IntoResponse};

pub enum AppError {
    NotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "resource not found"),
        };

        (status_code, message).into_response()
    }
}
