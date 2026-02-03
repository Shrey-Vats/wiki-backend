use std::collections::HashMap;

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    common::error::{AppError, NotFoundError},
    modules::todo::{
        model::{
            Category, CreateCategoryDto, CreateTagDto, NewTodo, TagDtoWithId, Tags, Todo, TodoResponse, UpdateTodoCredentials
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

    pub async fn create_todo(&self, user_id: Uuid, new: &NewTodo) -> Result<Todo, AppError> {
        let todo = TodoRepo::insert(&self.pool, user_id, new)
            .await
            .map_err(|_| AppError::DbError)?;

        Ok(todo)
    }

    pub async fn toggle(&self, todo_id: Uuid) -> Result<(), AppError> {
        TodoRepo::toggle(&self.pool, todo_id)
            .await
            .map_err(|_| AppError::DbError)?
            .ok_or_else(|| AppError::NotFound(NotFoundError::TodoNotFound))?;

        Ok(())
    }

    pub async fn list_todos(&self, user_id: Uuid) -> Result<Vec<TodoResponse>, AppError> {

        let rows = TodoRepo::fetch_all(&self.pool, user_id)
            .await
            .map_err(|_| AppError::DbError)?;

        let mut map: HashMap<Uuid, TodoResponse> = HashMap::new();

        for r in rows {
            let entry = map.entry(r.todo_id).or_insert(TodoResponse {
                id: r.todo_id,
                todo: r.todo_title,
                description: r.todo_description,
                category: CreateCategoryDto { name: r.category_name, slug: r.category_slug },
                is_done: r.is_done,
                created_at: r.created_at,
                tags: Vec::new()
            });

            if let (Some(_tag_id), Some(tag_name), Some(tag_slug)) = (r.tag_id, r.tag_name, r.tag_slug) {
                entry.tags.push(CreateTagDto { name: tag_name, slug: tag_slug });
            }
        }

        Ok(map.into_values().collect())
    }

    pub async fn get(&self, todo_id: Uuid) -> Result<TodoResponse, AppError> {
        let todo = TodoRepo::fetch(&self.pool, todo_id)
            .await
            .map_err(|_| AppError::DbError)?
            .ok_or_else(|| AppError::NotFound(NotFoundError::TodoNotFound))?;

        let tags = self.fetch_all_todo_tags(todo_id).await?;
        let category = self.fetch_category(&todo.category_id).await?;

        let return_todo = TodoResponse {
            id: todo.id,
            todo: todo.todo,
            description: todo.description,
            is_done: todo.is_done,
            tags: tags,
            category: category,
            created_at: todo.created_at,
        };

        Ok(return_todo)
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
        .await
        .map_err(|_| AppError::DbError)?;

        Ok(())
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

    pub async fn fetch_all_categories(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<CreateCategoryDto>, AppError> {
        let categories = TodoRepo::fetch_all_categories(&self.pool, user_id)
            .await
            .map_err(|_| AppError::DbError)?;

        Ok(categories)
    }

    pub async fn fetch_category(&self, category_id: &Uuid) -> Result<CreateCategoryDto, AppError> {
        let category = TodoRepo::fetch_category_id(&self.pool, category_id)
            .await?
            .ok_or_else(|| AppError::NotFound(NotFoundError::CategoryNotFound))?;

        Ok(category)
    }

    pub async fn fetch_category_slug(
        &self,
        user_id: Uuid,
        slug: &str,
    ) -> Result<CreateCategoryDto, AppError> {
        let category = TodoRepo::fetch_category(&self.pool, slug, user_id)
            .await
            .map_err(|_| AppError::DbError)?
            .ok_or_else(|| AppError::NotFound(NotFoundError::CategoryNotFound))?;

        let return_type = CreateCategoryDto {
            name: category.name,
            slug: category.slug,
        };

        Ok(return_type)
    }

    pub async fn delete_category(&self, slug: String, user_id: Uuid) -> Result<(), AppError> {
        TodoRepo::delete_categories(&self.pool, &slug, user_id)
            .await
            .map_err(|_| AppError::DbError)?;

        Ok(())
    }

    pub async fn fetch_all_todo_tags(&self, todo_id: Uuid) -> Result<Vec<CreateTagDto>, AppError> {
        let mut tags: Vec<CreateTagDto> = Vec::new();

        let all = TodoRepo::fetch_all_tag_todo(&self.pool, todo_id).await?;

        for v in all {
            let tag = TodoRepo::fetch_tag_id(&self.pool, v.tag_id)
                .await?
                .ok_or_else(|| AppError::NotFound(NotFoundError::TagNotFound))?;
            tags.push(tag);
        }

        Ok(tags)
    }

    pub async fn fetch_tag_slug(
        &self,
        user_id: Uuid,
        slug: &str,
    ) -> Result<TagDtoWithId, AppError> {
        let tag = TodoRepo::fetch_tag(&self.pool, slug, user_id)
            .await
            .map_err(|_| AppError::DbError)?
            .ok_or_else(|| AppError::NotFound(NotFoundError::CategoryNotFound))?;

        let return_type: TagDtoWithId = TagDtoWithId {
            id: tag.id,
            name: tag.name,
            slug: tag.slug,
        };

        Ok(return_type)
    }

    pub async fn fetch_tag(&self, tag_id: Uuid) -> Result<CreateTagDto, AppError> {
        let tag = TodoRepo::fetch_tag_id(&self.pool, tag_id)
            .await?
            .ok_or_else(|| AppError::NotFound(NotFoundError::CategoryNotFound))?;

        Ok(tag)
    }

    pub async fn create_tag_todo(&self, todo_id: &Uuid, tag_id: &Uuid) -> Result<(), AppError> {
        TodoRepo::create_tag_todo(&self.pool, todo_id, tag_id).await.map_err(|_| AppError::DbError)?;

        Ok(())
    }
}