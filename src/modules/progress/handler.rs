use axum::{Json, extract::{Path, State}};
use axum_macros::debug_handler;
use uuid::Uuid;

use crate::{
    common::{error::AppError, response::ApiResponse},
    modules::progress::{model::{DailyProgressDto, DailyProgressTodoDto}, service::ProgressService},
    state::AppState,
};

#[debug_handler]
pub async fn create_daily_progress_handler(
    State(state): State<AppState>,
    Json(dto): Json<DailyProgressDto>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    
    let daily_progress = state
        .progress_service
        .create_daily_progress(&dto.user_id, dto.day)
        .await?;

    Ok(Json(ApiResponse::success(
        "Today, canvas successfuly created",
        daily_progress,
    )))
}
pub async fn create_daily_progress_todo_handler(
    State(state): State<AppState>,
    Json(dto): Json<DailyProgressTodoDto>
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let daily_progress_todo = state.progress_service.create_daily_progress_todo(&dto.progress_id, &dto.todo_id, dto.is_done).await?;

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
    Path(progress_todo_id): Path<Uuid>
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let daily_progress_todo = ProgressService::toggle_daily_progress_todo(&state.progress_service, &progress_todo_id).await?;

    Ok(Json(ApiResponse::success("Toggle todo successfuly", daily_progress_todo)))
}
pub async fn fetch_all_daily_progress_todos(
        State(state): State<AppState>,
    Path(daily_progress_id): Path<Uuid>
)-> Result<Json<ApiResponse<impl serde::Serialize>>, AppError>  {
    let todos = ProgressService::fetch_all_daily_progress_todo(&state.progress_service, &daily_progress_id).await?;

    Ok(Json(ApiResponse::success("fetched all successfuly", todos)))
}