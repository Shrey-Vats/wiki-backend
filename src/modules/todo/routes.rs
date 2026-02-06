use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::{
    modules::{todo::handler::{
        create_category_handler, create_tag_handler, create_todo_handler, delete_category_handler, delete_tag_handler, delete_todo_handler, fetch_all_categories_handler, fetch_all_tags_handler, get_todo_handler, update_todo_handler
    }, user::handler::{delete_user_handler, get_user_handler}},
    state::AppState,
};

pub fn todo_routes() -> Router<AppState> {
    Router::new()
        .route("/todo/add", post(create_todo_handler))
        .route("/todo/update/{id}", put(update_todo_handler))
        .route("/todo/remove/{id}", delete(delete_todo_handler))
        .route("/todo/get/{id}", get(get_todo_handler))
        .route("/user/delete", delete(delete_user_handler))
        .route("/user/me", get(get_user_handler))
        .route("/tag/add", post(create_tag_handler))
        .route("/tag/{slug}", get(delete_tag_handler))
        .route("/tag/all", get(fetch_all_tags_handler))
        .route("/category/add", post(create_category_handler))
        .route("/category/{slug}", delete(delete_category_handler))
        .route("/category/all", get(fetch_all_categories_handler))
}