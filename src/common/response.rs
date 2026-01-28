use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub message: &'static str,
    pub success: bool,
    pub data: Option<T>,
}

pub fn ok<T>(message: &'static str, data: T, status: StatusCode) -> Response
where
    T: Serialize,
{
    (
        status,
        Json(ApiResponse::<T> {
            message,
            data: Some(data),
            success: true,
        }),
    )
        .into_response()
}

pub fn err(message: &'static str, status: StatusCode) -> Response {
    (
        status,
        Json(ApiResponse::<()> {
            message,
            data: None,
            success: false,
        }),
    )
        .into_response()
}
