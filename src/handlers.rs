use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response as AxumResponse},
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use axum_macros::debug_handler;
use uuid::Uuid;

use crate::{
    common::response::{ApiResponse, err, ok}, modules::todo::model::{CreateTodoDto, TodoResponse, UpdateTodoCredentials}, sql_query::{
        create_user_db, delete_todo, delete_user, fetch_all_todos, fetch_todo_by_id,
        find_user_by_email, get_user_by_id, insert_todo, toggle_todo, update_todo_record,
    }, state::{
        AppState, LoginCredentials, SignUpCredentials, User, UserId,
    }, utils::create_jwt_token
};

#[debug_handler]
pub async fn create_user(
    State(state): State<AppState>,
    cookies: CookieJar,
    Json(payload): Json<SignUpCredentials>,
) -> impl IntoResponse {
    match find_user_by_email(&state.pool, &payload.email).await {
        Ok(Some(_)) => return err("User already exist", StatusCode::BAD_REQUEST),
        Err(e) => {
            eprintln!("db error: {:?}", e);
            return err("Failed to validate user", StatusCode::INTERNAL_SERVER_ERROR);
        }
        _ => {}
    }

    match create_user_db(&state.pool, &payload.name, &payload.email, &payload.password).await {
        Ok(u) => {
            match create_jwt_token(u.id, state).await {
                Ok(jwt) => {
                    let jar = cookies.add(Cookie::build(("jwt", jwt)).http_only(true).path("/"));
                    (
                        StatusCode::CREATED,
                        jar,
                        Json(ApiResponse::<()> {
                            message: "user successfully created",
                            success: true,
                            data: None,
                        }),
                    ).into_response()
                }
                Err(e) => {
                    eprintln!("jwt error: {:?}", e);
                    err("Failed to create jwt token", StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Err(e) => {
            eprintln!("db error: {:?}", e);
            err("Failed to create user entry in database", StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn login_user(
    State(state): State<AppState>,
    cookies: CookieJar,
    Json(payload): Json<LoginCredentials>,
) -> impl IntoResponse {
    match find_user_by_email(&state.pool, &payload.email).await {
        Ok(Some(user)) => {
            if user.password != payload.password {
                return err("Invalid credentials", StatusCode::BAD_REQUEST);
            }

            match create_jwt_token(user.id, state).await {
                Ok(jwt) => {
                    let jar = cookies.add(Cookie::build(("jwt", jwt)).http_only(true).path("/"));
                    (
                        StatusCode::OK,
                        jar,
                        Json(ApiResponse::<()> {
                            message: "Successfully logged in",
                            success: true,
                            data: None,
                        }),
                    ).into_response()
                }
                Err(e) => {
                    eprintln!("jwt error: {:?}", e);
                    err("Failed to create jwt", StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Ok(None) => err("User not found", StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("db error: {:?}", e);
            err("Failed to find user in db", StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_user_handler(
    State(state): State<AppState>,
    cookies: CookieJar,
    Extension(user_id): Extension<UserId>,
) -> impl IntoResponse {
    match delete_user(&state.pool, user_id.0).await {
        Ok(_) => {
            let jar = cookies.remove(Cookie::from("jwt"));
            (
                StatusCode::OK,
                jar,
                Json(ApiResponse::<()> {
                    message: "User deleted successfully",
                    success: true,
                    data: None,
                }),
            ).into_response()
        }
        Err(e) => {
            eprintln!("delete error: {:?}", e);
            err("Failed to delete user", StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[debug_handler]
pub async fn create_todo_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Json(payload): Json<CreateTodoDto>,
) -> Result<AxumResponse, AxumResponse> {
    if payload.todo.len() < 5 || payload.description.len() < 5 {
        return Err(err("Fields must be at least 5 characters", StatusCode::BAD_REQUEST));
    }

    let todo = insert_todo(&state.pool, user_id.0, &payload.todo, &payload.description)
        .await
        .map_err(|_| err("Failed to create todo", StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(ok(
        "Todo created successfully",
        TodoResponse {
            id: todo.id,
            todo: todo.todo,
            description: todo.description,
            is_done: todo.is_done,
            created_at: todo.created_at,
        },
        StatusCode::CREATED,
    ))
}

pub async fn toggle_todo_handler(
    State(state): State<AppState>,
    Path(todo_id): Path<Uuid>,
) -> impl IntoResponse {
    match toggle_todo(&state.pool, todo_id).await {
        Ok(Some(todo)) => ok("Todo updated successfully", todo, StatusCode::OK),
        Ok(None) => err("Todo not found", StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("toggle error: {:?}", e);
            err("Failed to update todo state", StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn list_todos_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
) -> impl IntoResponse {
    match fetch_all_todos(&state.pool, user_id.0).await {
        Ok(value) => {
            let todos: Vec<TodoResponse> = value.into_iter().map(|t| TodoResponse {
                id: t.id,
                todo: t.todo,
                description: t.description,
                is_done: t.is_done,
                created_at: t.created_at,
            }).collect();

            ok("All todos fetched successfully", todos, StatusCode::OK)
        }
        Err(e) => {
            eprintln!("fetch error: {:?}", e);
            err("Failed to fetch todos", StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_todo_handler(
    State(state): State<AppState>,
    Path(todo_id): Path<Uuid>,
) -> impl IntoResponse {
    match delete_todo(&state.pool, todo_id).await {
        Ok(_) => ok("Todo successfully removed", (), StatusCode::OK),
        Err(e) => {
            eprintln!("delete error: {:?}", e);
            err("Failed to remove todo", StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_todo_handler(
    State(state): State<AppState>,
    Path(todo_id): Path<Uuid>,
) -> impl IntoResponse {
    match fetch_todo_by_id(&state.pool, todo_id).await {
        Ok(Some(todo)) => ok("Successfully fetched todo", todo, StatusCode::OK),
        Ok(None) => err("Invalid todo id", StatusCode::BAD_REQUEST),
        Err(e) => {
            eprintln!("fetch error: {:?}", e);
            err("Failed to fetch todo", StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[debug_handler]
pub async fn update_todo_handler(
    State(state): State<AppState>,
    Path(todo_id): Path<Uuid>,
    Json(payload): Json<UpdateTodoCredentials>,
) -> impl IntoResponse {
    match update_todo_record(
        &state.pool,
        todo_id,
        payload.todo.as_deref(),
        payload.description.as_deref(),
    ).await {
        Ok(todo) => ok("Updated successfully", todo, StatusCode::OK),
        Err(e) => {
            eprintln!("update error: {:?}", e);
            err("Failed to update todo", StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_user_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
) -> impl IntoResponse {
    match get_user_by_id(&state.pool, user_id.0).await {
        Ok(Some(user)) => ok("User data fetched successfully", user, StatusCode::OK),
        Ok(None) => err("Invalid user_id", StatusCode::BAD_REQUEST),
        Err(e) => {
            eprintln!("user fetch error: {:?}", e);
            err("Failed to fetch user", StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
