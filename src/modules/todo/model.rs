use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::modules::todo::errors::TodoValidationError;

#[derive(Debug, Clone, FromRow)]
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
            description: description.to_string()
        });
    }
}
