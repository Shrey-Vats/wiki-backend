use crate::{
    common::{
        error::{AppError, NotFoundError},
        response::ApiResponse,
    },
    modules::{
        rooms::{model::{Message, MessageDto, MessageResponse, RoomDto}, repository::RoomRepo},
        user::model::UserId,
    },
    state::AppState,
};
use axum::{
    Extension, Json,
    extract::{Path, State, WebSocketUpgrade, ws::WebSocket},
    http::StatusCode, response::IntoResponse,
};
use sqlx::{PgPool, pool};
use uuid::Uuid;

pub async fn create_room_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Json(dto): Json<RoomDto>,
) -> Result<(StatusCode, Json<ApiResponse<impl serde::Serialize>>), AppError> {
    let room = RoomDto::validate(dto)?;

    let room = RoomRepo::create_room(&state.pool, room, user_id.0).await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success("Room created successfully", room)),
    ))
}

pub async fn get_room_handler(
    State(state): State<AppState>,
    Path(room_id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<impl serde::Serialize>>), AppError> {
    match RoomRepo::get_room(&state.pool, room_id).await? {
        Some(value) => return Ok((StatusCode::OK, Json(ApiResponse::success("Successfully fetch room", value)))),
        None => return Err(AppError::NotFound(NotFoundError::RoomNotFound)),
    };
}

pub async fn get_all_rooms_handler(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<ApiResponse<impl serde::Serialize>>), AppError> {
    let rooms = RoomRepo::get_all_rooms(&state.pool).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Successfully fetch all rooms", rooms)),
    ))
}

pub async fn create_message(
    pool: &PgPool,
    user_id: Uuid,
    dto: MessageDto,
) -> Result<MessageResponse, AppError> {
    let message: MessageResponse = RoomRepo::create_message(pool, dto, user_id).await?;

    Ok(message)
}

pub async fn get_room_messages(
    pool: &PgPool,
    room_id: Uuid
) -> Result<Vec<MessageResponse>, AppError> {
    let message = RoomRepo::load_recent_messages(pool, room_id).await?;
    Ok(message)
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Path(room_id): Path<Uuid>,
    Extension(user_id): Extension<UserId>
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handler_socket(socket, state, room_id, user_id.0))
}

pub async fn handler_socket(
    socket: WebSocket,
    state: AppState,
    room_id: Uuid,
    user_id: Uuid
) {
    let mut rooms = state.rooms.lock().await;


}