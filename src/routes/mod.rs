
use axum::{Router, middleware::from_fn_with_state};

use crate::{
 middleware::auth::auth_middleware, modules::{progress::routes::progress_routes, todo::routes::todo_routes, user::routes::auth_router}, state::AppState
};

pub fn create_app(state: AppState) -> Router {
    Router::new()
    .nest("/api", progress_routes()).route_layer(from_fn_with_state(state.clone(), auth_middleware))
    .nest("/api", todo_routes()).route_layer(from_fn_with_state(state.clone(), auth_middleware))
    .nest("/api", auth_router())
    .with_state(state)
}
