use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::{
    modules::{todo::handler::{
        create_tag_handler, create_todo_handler, delete_todo_handler, get_todo_handler, list_todos_handler, toggle_todo_handler, update_todo_handler
    }, user::handler::{delete_user_handler, get_user_handler}},
    state::AppState,
};

pub fn todo_routes() -> Router<AppState> {
    Router::new()
        .route("/todo/all", get(list_todos_handler))
        .route("/todo/add", post(create_todo_handler))
        .route("/todo/toggle/{id}", put(toggle_todo_handler))
        .route("/todo/update/{id}", put(update_todo_handler))
        .route("/todo/remove/{id}", delete(delete_todo_handler))
        .route("/todo/get/{id}", get(get_todo_handler))
        .route("/user/delete", delete(delete_user_handler))
        .route("/user/me", get(get_user_handler))
        .route("todo/tag/add", post(create_tag_handler))
}
