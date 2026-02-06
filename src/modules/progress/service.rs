use sqlx::PgPool;
use time::Date;
use uuid::Uuid;

use crate::{
    common::error::AppError,
    modules::{
        progress::{
            model::{CompleteDailyProgressTodo, DailyProgress, DailyProgressTodo, ProgressTodoRespons},
            repository::ProgressRepo,
        },
        todo::repository::TodoRepo,
    },
};

#[derive(Debug, Clone)]
pub struct ProgressService {
    pub pool: PgPool,
}

impl ProgressService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool: pool }
    }

    pub async fn create_daily_progress(
        &self,
        user_id: &Uuid,
        day: Date,
    ) -> Result<DailyProgress, AppError> {
        let daily_progress = ProgressRepo::create_daily_progress(&self.pool, user_id, day)
            .await
            .map_err(|_| AppError::DbError)?;

        Ok(daily_progress)
    }

    pub async fn create_daily_progress_todo(
        &self,
        progress_id: &Uuid,
        todo_id: &Uuid,
        is_done: bool,
    ) -> Result<DailyProgressTodo, AppError> {
        let progress_todo =
            ProgressRepo::create_daily_progress_todo(&self.pool, todo_id, progress_id, is_done)
                .await
                .map_err(|_| AppError::DbError)?;

        Ok(progress_todo)
    }

    pub async fn toggle_daily_progress_todo(&self, progress_todo_id: &Uuid) -> Result<DailyProgressTodo, AppError>{
        let todo = ProgressRepo::toggle_daily_progress_todo(&self.pool, progress_todo_id).await.map_err(|_| AppError::DbError)?;

        Ok(todo)
    }

    pub async fn fetch_all_daily_progress_todo(&self, daily_progress_id: &Uuid) -> Result<Vec<CompleteDailyProgressTodo>, AppError> {
        let progress_todos= ProgressRepo::fetch_all_daily_progress_todos(&self.pool, daily_progress_id).await.map_err(|_| AppError::DbError)?;
        Ok(progress_todos)
    }

    pub async fn fetch_daily_progress_todo_id(&self, progress_todo_id: &Uuid)-> Result<ProgressTodoRespons, AppError> {
        let task: ProgressTodoRespons = ProgressRepo::fetch_daily_progress_todo_by_id(&self.pool, progress_todo_id).await.map_err(|_| AppError::DbError)?;

        Ok(task)
    }

    // pub async fn 
}

