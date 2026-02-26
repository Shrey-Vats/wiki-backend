use crate::{
    common::{
        error::{AppError, NotFoundError},
        response::ApiResponse,
    },
    modules::{
        rooms::{
            model::{
                ChatMessage, ClientEvent, MessageDto, MessageType, RoomDto,
                ServerEvent,
            },
            repository::RoomRepo,
        },
        user::{model::UserId, repository::UserRepo},
    },
    state::AppState,
};
use axum::{
    Extension, Json,
    extract::{
        Path, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    http::StatusCode,
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use tokio::sync::broadcast;
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
        Some(value) => {
            return Ok((
                StatusCode::OK,
                Json(ApiResponse::success("Successfully fetch room", value)),
            ));
        }
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

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Path(room_id): Path<Uuid>,
    Extension(user_id): Extension<UserId>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handler_socket(socket, state, room_id, user_id.0))
}

pub async fn handler_socket(socket: WebSocket, state: AppState, room_id: Uuid, user_id: Uuid) {
    let mut rooms = state.rooms.lock().await;

    let tx = rooms
        .entry(room_id.to_string())
        .or_insert_with(|| {
            let (tx, _rx) = broadcast::channel(100);
            tx
        })
        .clone();

    drop(rooms);

    let username = match UserRepo::fetch_by_id(&state.pool, user_id).await {
        Ok(Some(user)) => user.name,
        Ok(None) => {
            eprintln!("User not found for id: {user_id}");
            return;
        }
        Err(err) => {
            eprintln!("Failed to fetch user {user_id}: {err}");
            return;
        }
    };

    let mut rx = tx.subscribe();
    let (mut sender, mut receiver) = socket.split();

    // send history
    if let Ok(history) = RoomRepo::load_recent_messages(&state.pool, room_id).await {
        if let Ok(send) = serde_json::to_string(&ServerEvent::History(history)) {
            let _ = sender.send(Message::Text(send.into())).await;
        } else {
            eprintln!("Failed to convert history to string for room: {room_id}");
        }
    }

    // fan-out task
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            match serde_json::to_string(&ServerEvent::ChatMessage(msg)) {
                Err(_) => {
                    break;
                }
                Ok(v) => {
                    if sender.send(Message::Text(v.into())).await.is_err() {
                        break;
                    }
                }
            };
        }
    });

    // receive task
    let pool = state.pool.clone();
    let tx_for_recv = tx.clone();

    let mut recv_task = tokio::spawn(async move {
        let mut receiver = receiver;
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            if let Ok(evt) = serde_json::from_str::<ClientEvent>(&text) {
                match evt {
                    ClientEvent::ChatSend { content } => {
                        if let Ok(dto) = MessageDto::validate(MessageDto { room_id, content }) {
                            if let Ok(saved) = RoomRepo::create_message(&pool, dto, &user_id).await
                            {
                                let _ = tx_for_recv.send(ChatMessage {
                                    id: Some(saved.id),
                                    message_type: MessageType::User,
                                    user: username.clone(),
                                    message: saved.content,
                                    created_at: Some(saved.created_at),
                                });
                            }
                        }
                    }
                    ClientEvent::Ping => {
                        if let Ok(_) = serde_json::to_string(&ServerEvent::Pong) {
                            let _ = tx_for_recv.send(ChatMessage {
                                id: None,
                                message_type: MessageType::System,
                                user: "System".into(),
                                message: "pong".into(),
                                created_at: None,
                            });
                        }
                    }
                }
            }
        }
    });

    tokio::select! {
        _ = &mut send_task => {
            recv_task.abort()
        }
        _ = &mut recv_task => {
            send_task.abort()
        }
    }
}
