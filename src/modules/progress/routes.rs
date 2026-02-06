use axum::{Router, routing::{get, post, put}};

use crate::{
    modules::progress::handler::{
        create_daily_progress_handler,fetch_all_daily_progress_todos, create_daily_progress_todo_handler, fetch_daily_progress_todo_by_id, toggle_daily_progress_todo_handler
    },
    state::AppState,
};

pub fn progress_routes() -> Router<AppState> {
    Router::new()
        .route("/progress", post(create_daily_progress_handler))
        .route("/progress/todo", post(create_daily_progress_todo_handler))
        .route("/progress/todo/{progress_todo_id}", get(fetch_daily_progress_todo_by_id))
        .route("/progress/todo/{progress_todo_id}", put(toggle_daily_progress_todo_handler))
        .route("/progress/todos/{daily_progress_id}", get(fetch_all_daily_progress_todos))
}
