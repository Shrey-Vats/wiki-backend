use sqlx::{PgPool, Result};
use uuid::Uuid;

use crate::{common::error::AppError, modules::user::model::{User, UserResponseDto}};

pub struct UserRepo;

impl UserRepo {
    pub async fn create(pool: &PgPool, name: &str, email: &str, password: &str) -> Result<UserResponseDto> {
        let user = sqlx::query_as!(
            UserResponseDto,
            r#"
        INSERT INTO users (name, email, password)
        VALUES ($1, $2, $3)
        RETURNING id, name, email, is_public
        "#,
            name,
            email,
            password
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn fetch_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<UserResponseDto>> {
        let user = sqlx::query_as!(
            UserResponseDto,
            r#"
        SELECT id, name, email, is_public
        FROM users
        WHERE id = $1
        "#,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn delete(pool: &PgPool, user_id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query!(
            "
        DELETE FROM users
        WHERE id = $1
        ",
            user_id
        )
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::Failed("Failed to delete user".into()))
        }

        Ok(())
    }

    pub async fn fetch_by_email(pool: &PgPool, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
        SELECT id, name, email, password, is_public
        FROM users
        WHERE email = $1
        "#,
            email
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }
}
