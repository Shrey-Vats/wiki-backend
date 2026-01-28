
use axum::{
    Router, routing::{post}
};


use crate::{modules::user::handler::{create_user, login_user}, state::AppState};

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/user/create", post(create_user))
        .route("/user/login", post(login_user))
}