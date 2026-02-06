use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::{common::error::AppError, modules::todo::errors::TodoValidationError};

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Todo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category_id: Uuid,
    pub title: String,
    pub description: String,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct TodoCred {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub category_id: Uuid,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime
}

#[derive(Debug, Clone, Serialize)]
pub struct TodoResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub category: CreateCategoryDto,
    pub tags: Vec<CreateTagDto>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime
}

#[derive(FromRow, Serialize)]
pub struct Tags {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub slug: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateTagDto {
    pub name: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Debug,)]
pub struct TagDtoWithId {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub category_id: Uuid,
    pub tags: Vec<String>
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
    pub tags_slug: Vec<String>, 
    pub category_slug: String,
    pub category_id: Uuid
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
            category_id: value.category_id,
            tags: value.tags_slug
        });
    }
}
