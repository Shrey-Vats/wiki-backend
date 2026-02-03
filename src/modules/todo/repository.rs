use sqlx::{PgPool, Postgres, QueryBuilder, Result};
use uuid::Uuid;

use crate::{common::error::AppError, modules::todo::model::{
    Category, CreateCategoryDto, CreateTagDto, NewTodo, TagTodo, Tags, Todo, TodoCred, TodoFullRows,
}};

pub struct TodoRepo;

impl TodoRepo {
    pub async fn insert(pool: &PgPool, user_id: Uuid, new: &NewTodo) -> Result<Todo> {
        let todo = sqlx::query_as!(
            Todo,
            r#"
        INSERT INTO todos (user_id, todo, description, category_id)
        VALUES ($1, $2, $3, $4)
        RETURNING id, user_id, todo, description, is_done, created_at, category_id
        "#,
            user_id,
            new.todo,
            new.description,
            new.category_id
        )
        .fetch_one(pool)
        .await?;

        Ok(todo)
    }

    pub async fn toggle(pool: &PgPool, todo_id: Uuid) -> Result<Option<TodoCred>> {
        let todo = sqlx::query_as!(
            TodoCred,
            r#"
        UPDATE todos
        SET is_done = NOT is_done
        WHERE id = $1
        RETURNING id, todo, description, is_done, created_at, category_id
        "#,
            todo_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(todo)
    }

    pub async fn fetch_all(pool: &PgPool, user_id: Uuid) -> Result<Vec<TodoFullRows>> {
        let todos = sqlx::query_as!(
            TodoFullRows,
            r#"
        SELECT
            t.id AS todo_id,
            t.todo AS todo_title,
            t.description AS todo_description,
            t.is_done,
            t.created_at,

            c.id AS category_id,
            c.name AS category_name,
            c.slug AS category_slug,

            tg.id AS tag_id,
            tg.name AS tag_name,
            tg.slug AS tag_slug
        
        FROM todos t
        JOIN categories c ON c.id = t.category_id
        LEFT JOIN tag_todo tt ON tt.todo_id = t.id
        LEFT JOIN tags tg ON tg.id = tt.tag_id
        WHERE t.user_id = $1
        ORDER BY t.created_at DESC
        "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(todos)
    }

    pub async fn fetch(pool: &PgPool, todo_id: Uuid) -> Result<Option<TodoCred>> {
        let todo = sqlx::query_as!(
            TodoCred,
            r#"
        SELECT id, todo, description, is_done, created_at, category_id
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
    ) -> Result<TodoCred> {
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

        let updated_todo: TodoCred = qb.build_query_as().fetch_one(pool).await?;

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
        )
        .fetch_one(pool)
        .await?;

        Ok(tag)
    }

    pub async fn fetch_all_tags(pool: &PgPool, user_id: Uuid) -> Result<Vec<CreateTagDto>> {
        let tags: Vec<CreateTagDto> = sqlx::query_as!(
            CreateTagDto,
            r#"
            SELECT name, slug 
            FROM tags
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

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
        )
        .fetch_one(pool)
        .await?;

        Ok(())
    }

    pub async fn create_categories(
        pool: &PgPool,
        user_id: Uuid,
        category: CreateCategoryDto,
    ) -> Result<Category> {
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
        )
        .fetch_one(pool)
        .await?;

        Ok(category)
    }

    pub async fn fetch_all_categories(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<CreateCategoryDto>> {
        let categories = sqlx::query_as!(
            CreateCategoryDto,
            r#"
            SELECT name, slug
            FROM categories
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

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
        )
        .fetch_one(pool)
        .await?;

        Ok(())
    }

    pub async fn create_tag_todo(pool: &PgPool, todo_id: &Uuid, tag_id: &Uuid) -> Result<()> {
        sqlx::query_as!(
            TagTodo,
            r#"
            INSERT INTO tag_todo (todo_id, tag_id) 
            VALUES ($1, $2)
            "#,
            todo_id,
            tag_id
        )
        .fetch_one(pool)
        .await?;

        Ok(())
    }

    pub async fn fetch_all_tag_todo(pool: &PgPool, todo_id: Uuid)-> Result<Vec<TagTodo>, AppError> {
        let all_tag_todo = sqlx::query_as!(
            TagTodo,
            r#"
            SELECT todo_id, tag_id
            FROM tag_todo
            WHERE todo_id = $1
            "#,
            todo_id
        ).fetch_all(pool).await.map_err(|_| AppError::DbError)?;

        Ok(all_tag_todo)
    }

    pub async fn fetch_tag(pool: &PgPool, slug: &str, user_id: Uuid) -> Result<Option<Tags>> {
        let tag = sqlx::query_as!(
            Tags,
            r#"
            SELECT id, user_id, name, slug 
            From tags
            WHERE slug = $1 AND user_id = $2
            "#,
            slug,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(tag)
    }

    pub async fn fetch_tag_id(pool: &PgPool, tag_id: Uuid) -> Result<Option<CreateTagDto>, AppError> {
        let tag = sqlx::query_as!(
            CreateTagDto,
            r#"
            SELECT name, slug
            FROM tags
            WHERE id = $1
            "#,
            tag_id
        ).fetch_optional(pool).await.map_err(|_| AppError::DbError)?;


        Ok(tag)
    }

    pub async fn fetch_category_id(pool: &PgPool, category_id: &Uuid) -> Result<Option<CreateCategoryDto>, AppError> {
         let category: Option<CreateCategoryDto> = sqlx::query_as!(
            CreateCategoryDto,
            r#"
            SELECT name, slug
            FROM categories
            WHERE id = $1
            "#,
            category_id
        ).fetch_optional(pool).await.map_err(|_| AppError::DbError)?;

        Ok(category)
    }

    pub async fn fetch_category(
        pool: &PgPool,
        slug: &str,
        user_id: Uuid,
    ) -> Result<Option<Category>> {
        let category = sqlx::query_as!(
            Category,
            r#"
            SELECT id, user_id, name, slug 
            From tags
            WHERE slug = $1 AND user_id = $2
            "#,
            slug,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(category)
    }
}
