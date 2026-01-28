use axum::{
    Extension, Json,
    extract::{Path, State},
};

use axum_macros::debug_handler;
use uuid::Uuid;

use crate::{
    common::{error::AppError, response::ApiResponse}, modules::{todo::model::{CreateTodoDto, NewTodo, UpdateTodoCredentials}, user::model::UserId}, state::AppState
};

#[debug_handler]
pub async fn create_todo_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Json(dto): Json<CreateTodoDto>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    
    let new_todo: NewTodo = dto.try_into()?;
    let todo = state.todo_service.create_todo(user_id.0, new_todo).await?;

    Ok(Json(ApiResponse::success("User created Successfully", todo)))
}

pub async fn toggle_todo_handler(
    State(state): State<AppState>,
    Path(todo_id): Path<Uuid>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let todo = state.todo_service.toggle(todo_id).await?;

    Ok(Json(ApiResponse::success("Todo Status Updated successfully", todo)))
}

pub async fn list_todos_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let todos = state.todo_service.list_todos(user_id.0).await?;

    Ok(Json(ApiResponse::success("All todos fetch successfuly", todos)))
}

pub async fn delete_todo_handler(
    State(state): State<AppState>,
    Path(todo_id): Path<Uuid>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    state.todo_service.delete(todo_id).await?;

    Ok(Json(ApiResponse::success("Todo deleted successfuly", None::<()>)))
}

pub async fn get_todo_handler(
    State(state): State<AppState>,
    Path(todo_id): Path<Uuid>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let todo = state.todo_service.get(todo_id).await?;

    Ok(Json(ApiResponse::success("Todo fetch successfuly", todo)))
}

#[debug_handler]
pub async fn update_todo_handler(
    State(state): State<AppState>,
    Path(todo_id): Path<Uuid>,
    Json(update): Json<UpdateTodoCredentials>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let todo = state.todo_service.update(update, todo_id).await?;

    Ok(Json(ApiResponse::success("Todo fetch successfuly", todo)))
}