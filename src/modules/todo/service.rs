use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    common::error::{AppError, NotFoundError},
    modules::todo::{
        model::{
            Category, CreateCategoryDto, CreateTagDto, NewTodo, TagDtoWithId, TagTodo, Tags, Todo, TodoResponse, UpdateTodoCredentials
        },
        repository::TodoRepo,
    },
};

#[derive(Debug, Clone)]
pub struct TodoService {
    pub pool: PgPool,
}

impl TodoService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    pub async fn update(
        &self,
        update: UpdateTodoCredentials,
        todo_id: Uuid,
    ) -> Result<(), AppError> {
        TodoRepo::update(
            &self.pool,
            todo_id,
            update.todo.as_deref(),
            update.description.as_deref(),
        )
        .await?;

        Ok(())
    }

    pub async fn delete(&self, todo_id: Uuid) -> Result<(), AppError> {
        TodoRepo::delete(&self.pool, todo_id).await?;

        Ok(())
    }

    pub async fn create_tag(&self, user_id: Uuid, dto: CreateTagDto) -> Result<Tags, AppError> {
        let tag = TodoRepo::create_tag(&self.pool, user_id, dto).await?;

        Ok(tag)
    }

    pub async fn fetch_all_tags(&self, user_id: Uuid) -> Result<Vec<CreateTagDto>, AppError> {
        let tags = TodoRepo::fetch_all_tags(&self.pool, user_id).await?;

        Ok(tags)
    }

    pub async fn delete_tag(&self, slug: String, user_id: Uuid) -> Result<(), AppError> {
        TodoRepo::delete_tag(&self.pool, &slug, user_id).await?;

        Ok(())
    }

    pub async fn create_category(
        &self,
        user_id: Uuid,
        dto: CreateCategoryDto,
    ) -> Result<Category, AppError> {
        let category = CreateCategoryDto::validation(dto)?;

        let tag = TodoRepo::create_categories(&self.pool, user_id, category).await?;

        Ok(tag)
    }

    pub async fn fetch_all_categories(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<CreateCategoryDto>, AppError> {
        let categories = TodoRepo::fetch_all_categories(&self.pool, user_id).await?;

        Ok(categories)
    }

    pub async fn delete_category(&self, slug: String, user_id: Uuid) -> Result<(), AppError> {
        TodoRepo::delete_categories(&self.pool, &slug, user_id).await?;
        Ok(())
    }

    pub async fn fetch_all_todo_tags(&self, todo_id: Uuid) -> Result<Vec<TagTodo>, AppError> {
        let all: Vec<TagTodo> = TodoRepo::fetch_all_tag_todo(&self.pool, todo_id).await?;
        Ok(all)
    }
    
    pub async fn create_tag_todo(&self, todo_id: &Uuid, tag_id: &Uuid) -> Result<(), AppError> {
        TodoRepo::create_tag_todo(&self.pool, todo_id, tag_id).await?;
        Ok(())
    }
}
