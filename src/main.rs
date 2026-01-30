use std::{env};

use crate::{ modules::{todo::service::TodoService, user::service::UserService}, routes::create_app, state::AppState, utils::db::init_db_pool};
use axum::response::Result;
use dotenvy::dotenv;
use jsonwebtoken::{DecodingKey, EncodingKey};
use sqlx::PgPool;

mod middleware;
mod routes;
mod state;
mod utils;
mod common;
mod modules;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")?;
    let secret = env::var("JWT_SECRET")?;

    let pool: PgPool = init_db_pool(&db_url).await?;

    let state: AppState = AppState {
        pool: pool.clone(),
        jwt_decoding: DecodingKey::from_secret(secret.as_bytes()),
        jwt_encoding: EncodingKey::from_secret(secret.as_bytes()),
        todo_service: TodoService::new(pool.clone()),
        user_service: UserService::new(pool)
    };

    let app = create_app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
