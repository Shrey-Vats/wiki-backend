use axum::{Json, http::StatusCode, response::{IntoResponse, Response}};
use thiserror::Error;

use crate::{common::response::ApiResponse, modules::{todo::errors::TodoValidationError, user::errors::UserValidationError}};

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Validation(#[from] TodoValidationError),
    #[error("Database error")]
    DbError,
    #[error(transparent)]
    NotFound(#[from] NotFoundError),
    #[error(transparent)]
    UserValidation(#[from] UserValidationError)
}

#[derive(Debug, Error)]
pub enum NotFoundError {
     #[error("Todo don't found")]
    TodoNotFound,
     #[error("User Not found")]
    UserNotFound
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Validation(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            AppError::DbError => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".into()),
            AppError::NotFound(e) => (StatusCode::NOT_FOUND, e.to_string()),
            AppError::UserValidation(e) => (StatusCode::BAD_REQUEST, e.to_string())

        };

        (
            status,
            Json(ApiResponse::<()> {
                success: false,
                data:  None,
                message: Box::leak(message.into_boxed_str())
            })
        ).into_response()
    }
}