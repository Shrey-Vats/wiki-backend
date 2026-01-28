use sqlx::{Postgres, QueryBuilder, Result, postgres::PgPool};
use uuid::Uuid;

use crate::{modules::todo::model::{Todo, TodoResponse}, state::User};


pub async fn create_user_db(pool: &PgPool, name: &str, email: &str, password: &str) -> Result<User> {
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
    ).fetch_one(pool).await?;

    Ok(user)
}

pub async fn insert_todo(pool: &PgPool, user_id: Uuid, todo: &str, description: &str) -> Result<Todo> {
    let todo = sqlx::query_as!(
        Todo,
        r#"
        INSERT INTO todos (user_id, todo, description)
        VALUES ($1, $2, $3)
        RETURNING id, user_id, todo, description, is_done, created_at
        "#,
        user_id,
        todo,
        description
    ).fetch_one(pool).await?;

    Ok(todo)
}

pub async fn toggle_todo(pool: &PgPool, todo_id: Uuid) -> Result<Option<TodoResponse>> {
    let todo = sqlx::query_as!(
        TodoResponse,
        r#"
        UPDATE todos
        SET is_done = NOT is_done
        WHERE id = $1
        RETURNING id, todo, description, is_done, created_at
        "#,
        todo_id
    ).fetch_optional(pool).await?;

    Ok(todo)
} 

pub async fn fetch_all_todos(pool: &PgPool, user_id: Uuid) -> Result<Vec<Todo>> {
    let todos = sqlx::query_as!(
        Todo,
        r#"
        SELECT id, user_id, todo, description, is_done, created_at
        FROM todos
        WHERE user_id = $1
        "#,
        user_id
    ).fetch_all(pool).await?;

    Ok(todos)
}

pub async fn fetch_todo_by_id(pool: &PgPool, todo_id: Uuid) -> Result<Option<TodoResponse>> {
    let todo = sqlx::query_as!(
        TodoResponse,
        r#"
        SELECT id, todo, description, is_done, created_at
        FROM todos
        WHERE id = $1
        "#,
        todo_id
    ).fetch_optional(pool).await?;

    Ok(todo)
}

pub async fn get_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<User>> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, name, email, password
        FROM users
        WHERE id = $1
        "#,
        user_id
    ).fetch_optional(pool).await?;

    Ok(user)
}

pub async fn delete_user(pool: &PgPool, user_id: Uuid) -> Result<()> {
    sqlx::query_as!(
        User,
        r#"
        DELETE FROM users
        WHERE id = $1
        "#,
        user_id
    ).fetch_optional(pool).await?;

    Ok(())
}

pub async fn delete_todo(pool: &PgPool, todo_id: Uuid) -> Result<()> {
    sqlx::query_as!(
        Todo,
        r#"
        DELETE FROM todos
        WHERE id = $1
        "#,
        todo_id
    ).fetch_optional(pool).await?;

    Ok(())
}

pub async fn find_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, name, email, password
        FROM users
        WHERE email = $1
        "#,
        email
    ).fetch_optional(pool).await?;

    Ok(user)
}

pub async fn update_todo_record(pool: &PgPool,todo_id: Uuid, todo: Option<&str>, description: Option<&str>) -> Result<TodoResponse> {
    
    let mut qb: QueryBuilder<'_, Postgres> = QueryBuilder::new("UPDATE todos SET");

    let mut separated = qb.separated(", ");

    if let Some(v) = todo{
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