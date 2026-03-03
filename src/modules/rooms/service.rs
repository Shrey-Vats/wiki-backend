use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    common::error::AppError,
    modules::rooms::{
        model::{Member, MessageDto, MessageResponse},
        repository::RoomRepo,
    },
};

#[derive(Debug, Clone)]
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

    pub async fn get_room_messages(self, room_id: Uuid) -> Result<Vec<MessageResponse>, AppError> {
        let message = RoomRepo::load_recent_messages(&self.pool, room_id).await?;
        Ok(message)
    }

    pub async fn join_room(self, room_id: &Uuid, user_id: &Uuid) -> Result<(), AppError> {
        RoomRepo::join_room(&self.pool, room_id, user_id).await?;
        Ok(())
    }

    pub async fn leave_room(self, room_id: &Uuid, user_id: &Uuid) -> Result<(), AppError> {
        RoomRepo::leave_room(&self.pool, room_id, user_id).await?;
        Ok(())
    }

    pub async fn get_user_join_status(
        self,
        room_id: &Uuid,
        user_id: &Uuid,
    ) -> Result<bool, AppError> {
        let is_member = RoomRepo::is_member(&self.pool, user_id, room_id)
            .await?
            .ok_or_else(|| AppError::Failed("Failed to fetch user join status".into()))?;

        Ok(is_member)
    }
}
