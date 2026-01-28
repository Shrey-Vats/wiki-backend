use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool};
use uuid::Uuid;

use crate::modules::{todo::service::TodoService, user::service::UserService};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_encoding: EncodingKey,
    pub jwt_decoding: DecodingKey,
    pub todo_service: TodoService,
    pub user_service: UserService
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub user_id: Uuid,
    // pub role: String,
    pub exp: usize, // expiry timestamp
    pub iat: usize, // current timestamp
}


