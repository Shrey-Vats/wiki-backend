use sqlx::{PgPool, Postgres, QueryBuilder, Result};
use uuid::Uuid;

use crate::modules::todo::model::{NewTodo, Todo, TodoResponse};

pub struct TodoRepo;

impl TodoRepo {
    pub async fn insert(pool: &PgPool, user_id: Uuid, new: NewTodo) -> Result<Todo> {
        let todo = sqlx::query_as!(
            Todo,
            r#"
        INSERT INTO todos (user_id, todo, description)
        VALUES ($1, $2, $3)
        RETURNING id, user_id, todo, description, is_done, created_at
        "#,
            user_id,
            new.todo,
            new.description
        )
        .fetch_one(pool)
        .await?;

        Ok(todo)
    }

    pub async fn toggle(pool: &PgPool, todo_id: Uuid) -> Result<Option<TodoResponse>> {
        let todo = sqlx::query_as!(
            TodoResponse,
            r#"
        UPDATE todos
        SET is_done = NOT is_done
        WHERE id = $1
        RETURNING id, todo, description, is_done, created_at
        "#,
            todo_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(todo)
    }

    pub async fn fetch_all(pool: &PgPool, user_id: Uuid) -> Result<Vec<Todo>> {
        let todos = sqlx::query_as!(
            Todo,
            r#"
        SELECT id, user_id, todo, description, is_done, created_at
        FROM todos
        WHERE user_id = $1
        "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(todos)
    }

    pub async fn fetch(pool: &PgPool, todo_id: Uuid) -> Result<Option<TodoResponse>> {
        let todo = sqlx::query_as!(
            TodoResponse,
            r#"
        SELECT id, todo, description, is_done, created_at
        FROM todos
        WHERE id = $1
        "#,
            todo_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(todo)
    }

    pub async fn delete(pool: &PgPool, todo_id: Uuid) -> Result<()> {
        sqlx::query_as!(
            Todo,
            r#"
        DELETE FROM todos
        WHERE id = $1
        "#,
            todo_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(())
    }

    pub async fn update(
        pool: &PgPool,
        todo_id: Uuid,
        todo: Option<&str>,
        description: Option<&str>,
    ) -> Result<TodoResponse> {
        let mut qb: QueryBuilder<'_, Postgres> = QueryBuilder::new("UPDATE todos SET");

        let mut separated = qb.separated(", ");

        if let Some(v) = todo {
            separated.push("todo = ").push_bind(v);
        }

        if let Some(v) = description {
            separated.push("description = ").push_bind(v);
        }

        if todo.is_none() && description.is_none() {
            return Err(sqlx::Error::Protocol("No field to update".into()));
        }

        qb.push(" WHERE id = ").push_bind(todo_id);

        qb.push("RETURNING id, todo, description, is_done, created_at");

        let updated_todo: TodoResponse = qb.build_query_as().fetch_one(pool).await?;

        Ok(updated_todo)
    }
}
