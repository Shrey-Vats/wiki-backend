use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    common::error::{AppError, NotFoundError},
    modules::todo::{
        model::{Category, CreateCategoryDto, CreateTagDto, NewTodo, Tags, Todo, TodoResponse, UpdateTodoCredentials},
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

    pub async fn create_todo(&self, user_id: Uuid, new: NewTodo) -> Result<Todo, AppError> {
        let todo = TodoRepo::insert(&self.pool, user_id, new)
            .await
            .map_err(|_| AppError::DbError)?;

        Ok(todo)
    }

    pub async fn toggle(&self, todo_id: Uuid) -> Result<TodoResponse, AppError> {
        let todo = TodoRepo::toggle(&self.pool, todo_id)
            .await
            .map_err(|_| AppError::DbError)?
            .ok_or_else(|| AppError::NotFound(NotFoundError::TodoNotFound))?;

        Ok(todo)
    }

    pub async fn list_todos(&self, user_id: Uuid) -> Result<Vec<TodoResponse>, AppError> {
        let todos = TodoRepo::fetch_all(&self.pool, user_id)
            .await
            .map_err(|_| AppError::DbError)?;

        let return_todos: Vec<TodoResponse> = todos
            .iter()
            .map(|v| TodoResponse {
                id: v.id,
                todo: v.todo.clone(),
                description: v.description.clone(),
                is_done: v.is_done,
                created_at: v.created_at,
            })
            .collect();

        Ok(return_todos)
    }

    pub async fn get(&self, todo_id: Uuid) -> Result<TodoResponse, AppError> {
        let todo = TodoRepo::fetch(&self.pool, todo_id)
            .await
            .map_err(|_| AppError::DbError)?
            .ok_or_else(|| AppError::NotFound(NotFoundError::TodoNotFound))?;

        Ok(todo)
    }

    pub async fn update(
        &self,
        update: UpdateTodoCredentials,
        todo_id: Uuid,
    ) -> Result<TodoResponse, AppError> {
        let todo = TodoRepo::update(
            &self.pool,
            todo_id,
            update.todo.as_deref(),
            update.description.as_deref(),
        )
        .await
        .map_err(|_| AppError::DbError)?;

        Ok(todo)
    }

    pub async fn delete(&self, todo_id: Uuid) -> Result<(), AppError> {
        TodoRepo::delete(&self.pool, todo_id)
            .await
            .map_err(|_| AppError::DbError)?;

        Ok(())
    }

    pub async fn create_tag(&self, user_id: Uuid, dto: CreateTagDto) -> Result<Tags, AppError> {

        let tag = TodoRepo::create_tag(&self.pool, user_id, dto)
            .await
            .map_err(|_| AppError::DbError)?;

        Ok(tag)
    }

    pub async fn fetch_all_tags(&self, user_id: Uuid) -> Result<Vec<CreateTagDto>, AppError> {
        let tags = TodoRepo::fetch_all_tags(&self.pool, user_id)
            .await
            .map_err(|_| AppError::DbError)?;

        Ok(tags)
    }

    pub async fn delete_tag(&self, slug: String, user_id: Uuid) -> Result<(), AppError> {
        TodoRepo::delete_tag(&self.pool, &slug, user_id)
            .await
            .map_err(|_| AppError::DbError)?;

        Ok(())
    }

    pub async fn create_category(
        &self,
        user_id: Uuid,
        dto: CreateCategoryDto,
    ) -> Result<Category, AppError> {
        let category = CreateCategoryDto::validation(dto)?;

        let tag = TodoRepo::create_categories(&self.pool, user_id, category)
            .await
            .map_err(|_| AppError::DbError)?;


        Ok(tag)
    }

    pub async fn fetch_all_categories(&self, user_id: Uuid) -> Result<Vec<CreateCategoryDto>, AppError> {
        let categories = TodoRepo::fetch_all_categories(&self.pool, user_id)
            .await
            .map_err(|_| AppError::DbError)?;

        Ok(categories)
    }

    pub async fn delete_category(&self, slug: String, user_id: Uuid) -> Result<(), AppError> {
        TodoRepo::delete_categories(&self.pool, &slug, user_id)
            .await
            .map_err(|_| AppError::DbError)?;

        Ok(())
    }
}
