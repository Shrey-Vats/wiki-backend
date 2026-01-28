use std::{env};

use crate::{app::create_app, state::AppState, utils::init_db_pool};
use axum::response::Result;
use dotenvy::dotenv;
use jsonwebtoken::{DecodingKey, EncodingKey};
use sqlx::PgPool;

mod app;
mod handlers;
mod middleware;
mod router;
mod sql_query;
mod state;
mod utils;
mod helper;
mod common;
mod modules;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")?;
    let secret = env::var("JWT_SECRET")?;

    let pool: PgPool = init_db_pool(&db_url).await?;

    let state: AppState = AppState {
        pool,
        jwt_decoding: DecodingKey::from_secret(secret.as_bytes()),
        jwt_encoding: EncodingKey::from_secret(secret.as_bytes()),
    };

    let app = create_app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
