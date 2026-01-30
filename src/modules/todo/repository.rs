use sqlx::{PgPool, Postgres, QueryBuilder, Result};
use uuid::Uuid;

use crate::modules::todo::model::{Category, CreateCategoryDto, CreateTagDto, NewTodo, TagTodo, Tags, Todo, TodoResponse};

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

    //tags
    pub async fn create_tag(pool: &PgPool, user_id: Uuid, tag: CreateTagDto) -> Result<Tags> {
        let tag = sqlx::query_as!(
            Tags,
            r#"
            INSERT INTO tags (user_id, name, slug)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, name, slug 
            "#,
            user_id,
            tag.name,
            tag.slug
        ).fetch_one(pool).await?;

        Ok(tag)
    }

    pub async fn fetch_all_tags(pool: &PgPool, user_id: Uuid) -> Result<Vec<CreateTagDto>> {
        let tags = sqlx::query_as!(
            CreateTagDto,
            r#"
            SELECT name, slug 
            FROM tags
            WHERE user_id = $1
            "#,
            user_id
        ).fetch_all(pool).await?;

        Ok(tags)
    }

    pub async fn delete_tag(pool: &PgPool, slug: &str, user_id: Uuid) -> Result<()> {
        sqlx::query_as!(
            CreateTagDto,
            r#"
            DELETE FROM tags
            WHERE slug = $1 AND user_id = $2
            "#,
            slug,
            user_id
        ).fetch_one(pool).await?;

        Ok(())
    }

    pub async fn create_categories(pool: &PgPool, user_id: Uuid, category: CreateCategoryDto) -> Result<Category> {
        let category = sqlx::query_as!(
            Category,
            r#"
            INSERT INTO categories (user_id, name, slug)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, name, slug
            "#,
            user_id,
            category.name,
            category.slug
        ).fetch_one(pool).await?;

        Ok(category)
    }

    pub async fn fetch_all_categories(pool: &PgPool, user_id: Uuid) -> Result<Vec<CreateCategoryDto>> {
        let categories = sqlx::query_as!(
            CreateCategoryDto,
            r#"
            SELECT name, slug
            FROM categories
            WHERE user_id = $1
            "#,
            user_id
        ).fetch_all(pool).await?;

        Ok(categories)
    }

    pub async fn delete_categories(pool: &PgPool, slug: &str, user_id: Uuid) -> Result<()> {
        sqlx::query_as!(
            CreateCategoryDto,
            r#"
            DELETE FROM categories
            WHERE slug = $1 AND user_id = $2 
            "#,
            slug,
            user_id
        ).fetch_one(pool).await?;

        Ok(())
    }

    pub async fn create_tag_todo(pool: &PgPool, todo_id: Uuid, tag_id: Uuid ) -> Result<TagTodo> {
        let tag_todo = sqlx::query_as!(
            TagTodo,
            r#"
            INSERT INTO tag_todo (todo_id, tag_id) 
            VALUES ($1, $2)
            RETURNING todo_id, tag_id
            "#,
            todo_id,
            tag_id
        ).fetch_one(pool).await?;

        Ok(tag_todo)
    }

    pub async fn fetch_id_from_tag_slug(pool: &PgPool, slug: &str, user_id: Uuid) -> Result<Option<Tags>>{
        let tag = sqlx::query_as!(
            Tags,
            r#"
            SELECT id, user_id, name, slug 
            From tags
            WHERE slug = $1 AND user_id = $2
            "#,
            slug,
            user_id
        ).fetch_optional(pool).await?;

        Ok(tag)
    }

    pub async fn fetch_id_from_category_slug(pool: &PgPool, slug: &str, user_id: Uuid) -> Result<Option<Category>>{
        let category = sqlx::query_as!(
            Category,
            r#"
            SELECT id, user_id, name, slug 
            From tags
            WHERE slug = $1 AND user_id = $2
            "#,
            slug,
            user_id
        ).fetch_optional(pool).await?;

        Ok(category)
    }

}
