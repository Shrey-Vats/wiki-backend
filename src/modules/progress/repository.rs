use sqlx::{PgPool, Result};
use time::Date;
use uuid::Uuid;

use crate::{
    common::error::{AppError, NotFoundError},
    modules::{
        progress::model::{
            CompleteDailyProgressTodo, DailyProgress, DailyProgressTodo, DailyProgressTodoDto,
            DailyProgressTodoResponse, ProgressTodoRespons,
        },
        todo::model::Todo,
    },
};

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
        daily_progress_id: &Uuid,
        user_id: &Uuid,
        new_todo: DailyProgressTodoResponse,
    ) -> Result<DailyProgressTodoDto, AppError> {
        let mut tx = pool.begin().await?;

        let todos = sqlx::query_as!(
            Todo,
            r#"
            INSERT INTO todos (user_id, title, description, category_id)
            VALUES ($1, $2, $3, 
        (
            SELECT id
            FROM categories
            WHERE slug = $4 AND user_id =$1
            LIMIT 1
        )
            )
            RETURNING id, user_id, title, description, created_at, updated_at, category_id
            "#,
            user_id,
            new_todo.todo,
            new_todo.description,
            new_todo.category_slug
        )
        .fetch_one(&mut *tx)
        .await?;

        let exits = sqlx::query_scalar!(
            r#"
            SELECT 1
            FROM daily_progress
            WHERE id = $1 AND user_id = $2
            "#,
            daily_progress_id,
            user_id
        )
        .fetch_one(&mut *tx)
        .await?;

        if exits.is_none() {
            return Err(AppError::NotFound(NotFoundError::DailyProgressNotFound));
        }

        let daily_progress_todo = sqlx::query_as!(
            DailyProgressTodo,
            r#"
            INSERT INTO daily_progress_todos (todo_id, daily_progress_id, is_done)
            VALUES ($1, $2, false)
            RETURNING id, todo_id, daily_progress_id, is_done, created_at 
            "#,
            todos.id,
            daily_progress_id
        )
        .fetch_one(&mut *tx)
        .await?;

    tx.commit().await?;

        let return_value: DailyProgressTodoDto = DailyProgressTodoDto {
            id: todos.id,
            title: todos.title,
            description: todos.description,
            category_id: todos.category_id,
            is_done: daily_progress_todo.is_done,
            created_at: daily_progress_todo.created_at,
        };

        Ok(return_value)
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

    pub async fn toggle_daily_progress_todo(
        pool: &PgPool,
        id: &Uuid,
        user_id: &Uuid,
    ) -> Result<DailyProgressTodo> {
        let todo: DailyProgressTodo = sqlx::query_as!(
            DailyProgressTodo,
            r#"
            UPDATE daily_progress_todos dpt
            SET is_done = NOT dpt.is_done
            FROM daily_progress dp
            WHERE dpt.id = $1
            AND dpt.daily_progress_id = dp.id
            AND dp.user_id = $2
            RETURNING dpt.id, dpt.todo_id, dpt.daily_progress_id, dpt.is_done, dpt.created_at
            "#,
            id,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(todo)
    }

    pub async fn fetch_all_daily_progress_todos(
        pool: &PgPool,
        daily_progress_id: &Uuid,
    ) -> Result<Vec<CompleteDailyProgressTodo>> {
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
            c.slug AS category_slug,
            c.name AS category_name

  
        FROM daily_progress_todos t
        JOIN todos td ON td.id = t.todo_id
        JOIN categories c ON c.id = td.category_id
        WHERE t.daily_progress_id = $1
        ORDER BY t.created_at DESC
        "#,
            daily_progress_id
        )
        .fetch_all(pool)
        .await?;

    println!("all daily_progress todos: {:?}", todos);

        Ok(todos)
    }

    pub async fn get_progress_id(
        pool: &PgPool,
        user_id: &Uuid,
        day: Date,
    ) -> Result<Option<Uuid>, AppError> {
        let progress_id = sqlx::query_scalar!(
            r#"
        SELECT id
        FROM daily_progress
        WHERE user_id = $1 AND day = $2
        "#,
            user_id,
            day
        )
        .fetch_optional(pool)
        .await?;

        Ok(progress_id)
    }
}
