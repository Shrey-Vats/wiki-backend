
use axum::{Router, middleware::from_fn_with_state};

use crate::{
 middleware::auth::auth_middleware, router::{api_router, protected_router}, state::AppState
};

pub fn create_app(state: AppState) -> Router {
    Router::new()
    .nest("/api", protected_router()).route_layer(from_fn_with_state(state.clone(), auth_middleware))
    .nest("/api", api_router())
    .with_state(state)
}
