/*
room -> create, update, delete, get, get all, get all members
message -> create, update, delete, get all(perticular channel in descending)
member -> join, remove, unjoin
*/

use sqlx::{PgPool, Result};
use uuid::Uuid;

use crate::modules::rooms::model::{Member, Message, MessageDto, MessageResponse, Room, RoomDto};

pub struct RoomRepo;

impl RoomRepo {
    pub async fn create_room(pool: &PgPool, room: RoomDto, user_id: Uuid) -> Result<Room> {
        let room = sqlx::query_as!(
            Room,
            r#"
            INSERT INTO rooms (owner_id, name, description, profile_Pic)
            VALUES ($1, $2, $3, $4)
            RETURNING id, owner_id, name, description, profile_Pic, created_at
            "#,
            user_id,
            room.name,
            room.description,
            room.profile_pic
        )
        .fetch_one(pool)
        .await?;

        Ok(room)
    }
    //TODO: implment update, delete
    pub async fn get_room(pool: &PgPool, room_id: Uuid) -> Result<Option<Room>> {
        let room = sqlx::query_as!(
            Room,
            r#"
            SELECT id, owner_id, name, description, profile_pic, created_at
            FROM rooms
            WHERE id = $1
            "#,
            room_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(room)
    }

    pub async fn get_all_rooms(pool: &PgPool) -> Result<Vec<Room>> {
        let rooms = sqlx::query_as!(
            Room,
            r#"
            SELECT id, owner_id, name, description, profile_pic, created_at
            FROM rooms
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(rooms)
    }

    pub async fn create_message(
        pool: &PgPool,
        message: MessageDto,
        user_id: &Uuid,
    ) -> Result<MessageResponse> {
        let message: MessageResponse = sqlx::query_as!(
            MessageResponse,
            r#"
            WITH inserted AS (
                INSERT INTO user_messages (user_id, room_id, content)
                VALUES ($1, $2, $3)
                RETURNING id, user_id, content, created_at
            )
            
            SELECT 
                inserted.id,
                u.name as "user_name",
                inserted.content,
                inserted.created_at
            FROM inserted
            JOIN users u ON inserted.user_id = u.id
            "#,
            user_id,
            message.room_id,
            message.content
        )
        .fetch_one(pool)
        .await?;

        Ok(message)
    }

    //TODO: implement update, delete fn their

    pub async fn load_recent_messages(pool: &PgPool, room_id: Uuid) -> Result<Vec<MessageResponse>> {
        let message: Vec<MessageResponse> = sqlx::query_as!(
            MessageResponse,
            r#"
        SELECT 
            m.id,
            u.name as "user_name!",
            m.content,
            m.created_at
        FROM user_messages m
        JOIN users u ON m.user_id = u.id
        WHERE m.room_id = $1
        ORDER BY m.created_at DESC
        LIMIT 50
            "#,
            room_id
        )
        .fetch_all(pool)
        .await?;

        Ok(message)
    }

    pub async fn join_room(pool: &PgPool, room_id: Uuid, user_id: Uuid) -> Result<Member> {
        let member = sqlx::query_as!(
            Member,
            r#"
            INSERT INTO members (user_id, room_id)
            VALUES ($1, $2)
            RETURNING user_id, room_id
            "#,
            user_id,
            room_id
        )
        .fetch_one(pool)
        .await?;

        Ok(member)
    }
}
