use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    common::error::AppError,
    modules::rooms::{
        model::{MessageDto, MessageResponse},
        repository::RoomRepo,
    },
};

pub struct RoomService {
    pub pool: PgPool,
}

impl RoomService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_message(
        self,
        message_dto: MessageDto,
        user_id: &Uuid,
    ) -> Result<MessageResponse, AppError> {
        let message = RoomRepo::create_message(&self.pool, message_dto, user_id).await?;

        Ok(message)
    }

    pub async fn get_room_messages(
        pool: &PgPool,
        room_id: Uuid,
    ) -> Result<Vec<MessageResponse>, AppError> {
        let message = RoomRepo::load_recent_messages(pool, room_id).await?;
        Ok(message)
    }
}
