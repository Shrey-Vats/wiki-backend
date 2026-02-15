use axum::{
    Router,
    extract::ws,
    routing::{get, post},
};

use crate::{modules::rooms::handler::{create_room_handler, get_all_rooms_handler, get_room_handler, ws_handler}, state::AppState};

pub fn room_routes() -> Router<AppState> {
    Router::new()
        .route("/room", post(create_room_handler))
        .route("/room/info/{room_id}", get(get_room_handler))
        .route("/rooms", get(get_all_rooms_handler))
        .route("/room/{room_id}", get(ws_handler))
}
