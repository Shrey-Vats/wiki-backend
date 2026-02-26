use axum::{Extension, Json, extract::{Path, State}};
use axum_macros::debug_handler;
use uuid::Uuid;

use crate::{
    common::{error::AppError, response::ApiResponse},
    modules::{progress::{model::{DailyProgressDto, DailyProgressTodoResponse}, service::ProgressService}, user::model::UserId},
    state::AppState,
};

#[debug_handler]
pub async fn create_daily_progress_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Json(dto): Json<DailyProgressDto>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    
    let daily_progress = state
        .progress_service
        .create_daily_progress(&user_id.0, dto.day)
        .await?;

    Ok(Json(ApiResponse::success(
        "Today, canvas successfuly created",
        daily_progress,
    )))
}
pub async fn create_daily_progress_todo_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(daily_progress_id): Path<Uuid>,
    Json(dto): Json<DailyProgressTodoResponse>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let daily_progress_todo = state.progress_service.create_daily_progress_todo(&daily_progress_id, &user_id.0, dto).await?;

    Ok(Json(ApiResponse::success("Successfuly created progress todo", daily_progress_todo)))
}

pub async fn fetch_daily_progress_todo_by_id(
    State(state): State<AppState>,
    Path(progress_todo_id): Path<Uuid>
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let daily_progress_todo = ProgressService::fetch_daily_progress_todo_id(&state.progress_service, &progress_todo_id).await?;

    Ok(Json(ApiResponse::success("Todo updated successfuly", daily_progress_todo)))

}
pub async fn toggle_daily_progress_todo_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(progress_todo_id): Path<Uuid>
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let daily_progress_todo = ProgressService::toggle_daily_progress_todo(&state.progress_service, &progress_todo_id, &user_id.0).await?;


    Ok(Json(ApiResponse::success("Toggle todo successfuly", daily_progress_todo)))
}
pub async fn fetch_all_daily_progress_todos(
        State(state): State<AppState>,
    Path(daily_progress_id): Path<Uuid>
)-> Result<Json<ApiResponse<impl serde::Serialize>>, AppError>  {
    let todos = ProgressService::fetch_all_daily_progress_todo(&state.progress_service, &daily_progress_id).await?;

    Ok(Json(ApiResponse::success("fetched all successfuly", todos)))
}