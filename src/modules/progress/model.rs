use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use time::{Date, PrimitiveDateTime};


#[derive(Debug, FromRow, Serialize)]
pub struct DailyProgress {
    pub id: Uuid,
    pub user_id: Uuid,
    pub day: Date,

    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime
}

#[derive(Debug, FromRow, Serialize)]
pub struct DailyProgressTodo {
    pub id: Uuid,
    pub todo_id: Uuid,
    pub daily_progress_id: Uuid,
    pub is_done: bool,
    pub created_at: PrimitiveDateTime
}

#[derive(Debug, Serialize)]
pub struct DailyProgressTodoDto {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub category_id: Uuid,
    pub is_done: bool,
    pub created_at: PrimitiveDateTime
}

#[derive(Debug, Deserialize)]
pub struct DailyProgressTodoResponse {
    pub todo: String,
    pub description: String,
    pub category_slug: String,
}

#[derive(Debug, Serialize)]
pub struct ProgressTodoRespons {
    pub progress_todo_id: Uuid,
    pub todo_id: Uuid,
    pub daily_progress_id: Uuid,
    pub title: String,
    pub description: String,
    pub is_done: bool,
    pub created_at: PrimitiveDateTime
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct CompleteDailyProgressTodo {
    pub daily_progress_todo_id: Uuid,
    pub todo_id: Uuid,
    pub todo_title: String,
    pub todo_description: String,
    pub is_done: bool,
    pub created_at: PrimitiveDateTime,

    pub category_id: Uuid,
    pub category_name: String,
    pub category_slug: String,

    pub tag_id: Option<Uuid>,
    pub tag_name: Option<String>,
    pub tag_slug: Option<String>
}
// pub struct

#[derive(Debug, Deserialize)]
pub struct DailyProgressDto {
    pub day: String
}

// #[derive(Debug, Deserialize)]
// pub struct DailyProgressTodoDto {
//     pub todo_id: Uuid,
//     pub is_done: bool
// }
