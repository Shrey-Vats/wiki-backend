use sqlx::{PgPool, Result};
use time::Date;
use uuid::Uuid;

use crate::modules::progress::model::{CompleteDailyProgressTodo, DailyProgress, DailyProgressTodo, ProgressTodoRespons};

pub struct ProgressRepo;

impl ProgressRepo {
    pub async fn create_daily_progress(
        pool: &PgPool,
        user_id: &Uuid,
        day: Date,
    ) -> Result<DailyProgress> {
        let progress: DailyProgress = sqlx::query_as!(
            DailyProgress,
            r#"
            INSERT INTO daily_progress (user_id, day)
            VALUES ($1, $2)
            RETURNING id, user_id, day, created_at, updated_at
            "#,
            user_id,
            day
        )
        .fetch_one(pool)
        .await?;

        Ok(progress)
    }

    pub async fn fetch_daily_progress_by_user_id_and_day(
        pool: &PgPool,
        day: &Date,
        user_id: &Uuid,
    ) -> Result<DailyProgress> {
        let progress = sqlx::query_as!(
            DailyProgress,
            r#"
            SELECT id, user_id, day, created_at, updated_at
            FROM daily_progress
            WHERE user_id = $1 AND day = $2
            "#,
            user_id,
            day
        )
        .fetch_one(pool)
        .await?;

        Ok(progress)
    }

    pub async fn fetch_daily_progress_by_id(pool: &PgPool, id: &Uuid) -> Result<DailyProgress> {
        let progress = sqlx::query_as!(
            DailyProgress,
            r#"
            SELECT id, user_id, day, created_at, updated_at
            FROM daily_progress 
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(progress)
    }

    pub async fn create_daily_progress_todo(
        pool: &PgPool,
        todo_id: &Uuid,
        daily_progress_id: &Uuid,
        is_done: bool,
    ) -> Result<DailyProgressTodo> {
        let todo = sqlx::query_as!(
            DailyProgressTodo,
            r#"
            INSERT INTO daily_progress_todos (todo_id, daily_progress_id, is_done)
            VALUES ($1, $2, $3)
            RETURNING id, todo_id, daily_progress_id, is_done, created_at 
            "#,
            todo_id,
            daily_progress_id,
            is_done
        )
        .fetch_one(pool)
        .await?;

        Ok(todo)
    }

    pub async fn fetch_daily_progress_todo_by_id(
        pool: &PgPool,
        id: &Uuid,
    ) -> Result<ProgressTodoRespons> {
        let todo = sqlx::query_as!(
            ProgressTodoRespons,
            r#"
            SELECT pt.id AS progress_todo_id, pt.todo_id, pt.daily_progress_id, pt.is_done, pt.created_at, t.title, t.description
            FROM daily_progress_todos pt
            JOIN todos t ON pt.todo_id = t.id
            WHERE pt.id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(todo)
    }

    pub async fn toggle_daily_progress_todo(pool: &PgPool, id: &Uuid) -> Result<DailyProgressTodo> {
        let todo: DailyProgressTodo = sqlx::query_as!(
            DailyProgressTodo,
            r#"
            UPDATE daily_progress_todos
            SET is_done = NOT is_done
            WHERE id = $1
            RETURNING id, todo_id, daily_progress_id, is_done, created_at
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(todo)
    }

    pub async fn fetch_all_daily_progress_todos(pool: &PgPool, daily_progress_id: &Uuid) -> Result<Vec<CompleteDailyProgressTodo>> {
        let todos = sqlx::query_as!(
            CompleteDailyProgressTodo,
            r#"
            SELECT
            t.id AS daily_progress_todo_id,
            t.is_done,
            t.created_at,
            td.id as todo_id,
            td.title AS todo_title,
            td.description AS todo_description,
            c.id AS category_id,
            c.name AS category_name,
            c.slug AS category_slug,
            tg.id AS tag_id,
            tg.name AS tag_name,
            tg.slug AS tag_slug       
        FROM daily_progress_todos t
        JOIN todos td ON td.id = t.todo_id
        JOIN categories c ON c.id = td.category_id
        LEFT JOIN tag_todo tt ON tt.todo_id = td.id
        LEFT JOIN tags tg ON tg.id = tt.tag_id
        WHERE t.daily_progress_id = $1
        ORDER BY t.created_at DESC
        "#,
        daily_progress_id
        ).fetch_all(pool).await?;

        Ok(todos)
    }

}
