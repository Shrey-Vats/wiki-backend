
use axum::{
    Router, routing::{delete, get, post, put}
};

use crate::{handlers::{create_todo_handler, create_user, delete_todo_handler, delete_user_handler, get_todo_handler, get_user_handler, list_todos_handler, login_user, toggle_todo_handler, update_todo_handler }, state::AppState};

pub fn api_router() -> Router<AppState> {
    Router::new()
        .route("/user/create", post(create_user))
        .route("/user/login", post(login_user))
}

pub fn protected_router() -> Router<AppState> {
       Router::new()
        .route("/todo/all", get(list_todos_handler)) 
        .route("/todo/add", post(create_todo_handler)) 
        .route("/todo/toggle/{id}", put(toggle_todo_handler)) 
        .route("/todo/update/{id}", put(update_todo_handler))
        .route("/todo/remove/{id}", delete(delete_todo_handler))
        .route("/todo/get/{id}", get(get_todo_handler))
        .route("/user/delete", delete(delete_user_handler))
        .route("/user/me", get(get_user_handler))
}
