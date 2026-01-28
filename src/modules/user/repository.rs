use sqlx::{PgPool, Result};
use uuid::Uuid;

use crate::modules::user::model::User;

pub struct UserRepo;

impl UserRepo {
    pub async fn create(pool: &PgPool, name: &str, email: &str, password: &str) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
        INSERT INTO users (name, email, password)
        VALUES ($1, $2, $3)
        RETURNING id, name, email, password
        "#,
            name,
            email,
            password
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn fetch_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
        SELECT id, name, email, password
        FROM users
        WHERE id = $1
        "#,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn delete(pool: &PgPool, user_id: Uuid) -> Result<()> {
        sqlx::query_as!(
            User,
            r#"
        DELETE FROM users
        WHERE id = $1
        "#,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(())
    }

    pub async fn fetch_by_email(pool: &PgPool, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
        SELECT id, name, email, password
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
