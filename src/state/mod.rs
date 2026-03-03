use std::{collections::{HashMap, HashSet}, sync::Arc};

use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool};
use uuid::Uuid;
use tokio::sync::{Mutex, broadcast, mpsc};

use crate::modules::{progress::service::ProgressService, rooms::{model::{ServerEvent}, service::RoomService}, todo::service::TodoService, user::service::UserService};

type Username = String;
type RoomId = uuid::Uuid;
type UserId = uuid::Uuid;

#[derive(Clone)]
pub struct Member {
    username: String,
    tx: mpsc::Sender<ServerEvent>
}

#[derive(Clone)]
pub struct RoomState {
    pub members: HashMap<UserId, Member>
}

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_encoding: EncodingKey,
    pub jwt_decoding: DecodingKey,
    pub todo_service: TodoService,
    pub user_service: UserService,
    pub progress_service: ProgressService,
    pub room_service: RoomService,
    pub rooms: Arc<Mutex<HashMap<RoomId, RoomState>>>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub user_id: Uuid,
    // pub role: String,
    pub exp: usize, // expiry timestamp
    pub iat: usize, // current timestamp
}


