use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::{common::error::AppError, modules::todo::errors::TodoValidationError};

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Todo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub todo: String,
    pub description: String,
    pub is_done: bool,
    pub created_at: PrimitiveDateTime,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct TodoResponse {
    pub id: Uuid,
    pub todo: String,
    pub description: String,
    pub is_done: bool,
    pub created_at: PrimitiveDateTime,
}

#[derive(FromRow, Serialize)]
pub struct Tags {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub slug: String,
}

#[derive(Serialize)]
pub struct CreateTagDto {
    pub name: String,
    pub slug: String,
}

#[derive(Serialize)]
pub struct TagSlug {
    pub slug: String
}

#[derive(Serialize)]
pub struct CategorySlug {
    pub slug: String
}

#[derive(Serialize)]
pub struct CreateCategoryDto {
    pub name: String,
    pub slug: String,
}

#[derive(FromRow, Serialize)]
pub struct Category {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub slug: String,
}

#[derive(FromRow)]
pub struct TagTodo {
    pub todo_id: Uuid,
    pub tag_id: Uuid,
}

pub struct NewTodo {
    pub todo: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTodoCredentials {
    pub todo: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTodoDto {
    pub todo: String,
    pub description: String,
    pub is_done: bool,
}

impl CreateTagDto {
    pub fn validate(dto: CreateTagDto) -> Result<Self, AppError> {
        let name = dto.name.trim();
        let slug = dto.slug.trim();

        if name.len() < 3 {
            return Err(AppError::Validation(TodoValidationError::InvalidTag));
        }
        if slug.len() < 3 {
            return Err(AppError::Validation(TodoValidationError::InvalidTag));
        }

        Ok(Self {
            name: name.to_string(),
            slug: slug.to_string(),
        })
    }
}

impl CreateCategoryDto {
    pub fn validation(dto: CreateCategoryDto) -> Result<Self, AppError> {
         let name = dto.name.trim();
        let slug = dto.slug.trim();

        if name.len() < 3 {
            return Err(AppError::Validation(TodoValidationError::InvalidTag));
        }
        if slug.len() < 3 {
            return Err(AppError::Validation(TodoValidationError::InvalidTag));
        }

        Ok(Self {
            name: name.to_string(),
            slug: slug.to_string(),
        })
    }
}

impl TryFrom<CreateTodoDto> for NewTodo {
    type Error = TodoValidationError;

    fn try_from(value: CreateTodoDto) -> Result<Self, Self::Error> {
        let todo = value.todo.trim();
        let description = value.description.trim();

        if todo.len() < 5 {
            return Err(TodoValidationError::TodoTooShort);
        };

        if description.len() < 5 {
            return Err(TodoValidationError::DescriptionTooShort);
        };

        return Ok(Self {
            todo: todo.to_string(),
            description: description.to_string(),
        });
    }
}
