use axum::{
    Extension, Json,
    extract::{Path, State},
};

use axum_macros::debug_handler;
use uuid::Uuid;

use crate::{
    common::{error::AppError, response::ApiResponse},
    modules::{
        todo::{
            model::{
                CreateCategoryDto, CreateTagDto, CreateTodoDto, NewTodo,
                TodoResponse, UpdateTodoCredentials,
            },
            service::TodoService,
        },
        user::{model::UserId},
    },
    state::AppState,
};

#[debug_handler]
pub async fn create_todo_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Json(dto): Json<CreateTodoDto>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let new_todo: NewTodo = dto.try_into()?;
    let mut tags: Vec<CreateTagDto> = Vec::new();

    let todo = state.todo_service.create_todo(user_id.0, &new_todo).await?;

    for i in new_todo.tags {
        let tag = state.todo_service.fetch_tag_slug(user_id.0, &i).await?;
        
        state.todo_service.create_tag_todo(&todo.id, &tag.id).await?;
        
        let create_dto = CreateTagDto {
            name: tag.name,
            slug: tag.slug
        };

        tags.push(create_dto);
    }

    let category = state.todo_service.fetch_category(&todo.category_id).await?;

    let todo_response = TodoResponse {
        id: todo.id,
        todo: todo.todo,
        description: todo.description,
        is_done: todo.is_done,
        category: category,
        tags: tags,
        created_at: todo.created_at,
    };

    Ok(Json(ApiResponse::success(
        "User created Successfully",
        todo_response,
    )))
}

pub async fn toggle_todo_handler(
    State(state): State<AppState>,
    Path(todo_id): Path<Uuid>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    state.todo_service.toggle(todo_id).await?;

    Ok(Json(ApiResponse::success(
        "Todo Status Updated successfully",
        None::<()>,
    )))
}

pub async fn list_todos_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let todos = state.todo_service.list_todos(user_id.0).await?;

    Ok(Json(ApiResponse::success(
        "All todos fetch successfuly",
        todos,
    )))
}

pub async fn delete_todo_handler(
    State(state): State<AppState>,
    Path(todo_id): Path<Uuid>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    state.todo_service.delete(todo_id).await?;

    Ok(Json(ApiResponse::success(
        "Todo deleted successfuly",
        None::<()>,
    )))
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
     state.todo_service.update(update, todo_id).await?;

    Ok(Json(ApiResponse::success("Todo fetch successfuly", None::<()>)))
}

#[debug_handler]
pub async fn create_tag_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Json(dto): Json<CreateTagDto>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let tag = CreateTagDto::validate(dto)?;

    let service_tag = TodoService::create_tag(&state.todo_service, user_id.0, tag).await?;

    Ok(Json(ApiResponse::success(
        "Tag created successfuly",
        service_tag,
    )))
}

pub async fn fetch_all_tags_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let tags = TodoService::fetch_all_tags(&state.todo_service, user_id.0).await?;

    Ok(Json(ApiResponse::success(
        "All todos fetch successfuly",
        tags,
    )))
}

pub async fn delete_tag_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(slug): Path<String>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    TodoService::delete_tag(&state.todo_service, slug, user_id.0).await?;

    Ok(Json(ApiResponse::success(
        "Tag successfuly deleted",
        None::<()>,
    )))
}

pub async fn create_category_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Json(dto): Json<CreateCategoryDto>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let new_category_playload = CreateCategoryDto::validation(dto)?;

    let category =
        TodoService::create_category(&state.todo_service, user_id.0, new_category_playload).await?;

    Ok(Json(ApiResponse::success(
        "Category created successfuly",
        category,
    )))
}

pub async fn fetch_all_categories_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let categories = TodoService::fetch_all_categories(&state.todo_service, user_id.0).await?;

    Ok(Json(ApiResponse::success(
        "Fetch all categories successfully",
        categories,
    )))
}

pub async fn delete_category_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(slug): Path<String>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    TodoService::delete_category(&state.todo_service, slug, user_id.0).await?;
    Ok(Json(ApiResponse::success(
        "Category deleted successfuly",
        None::<()>,
    )))
}