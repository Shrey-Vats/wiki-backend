use std::{ time::Duration};

use chrono::{Duration as DurationC, Utc};
use jsonwebtoken::{Header, TokenData, Validation, decode, encode, errors::Result};
use sqlx::{PgPool, Result as Result_db, postgres::PgPoolOptions};
use uuid::Uuid;

use crate::state::{AppState, Claims};

pub async fn create_jwt_token(user_id: Uuid, state: AppState) -> Result<String> {
    let now = Utc::now();

    let claims = Claims {
        user_id: user_id,
        iat: now.timestamp() as usize,
        exp: (now + DurationC::days(7)).timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &state.jwt_encoding,
    )?;

    Ok(token)
}

pub fn verify_jwt_token(token: &str, state: AppState) -> Result<Claims> {
    let claims: TokenData<Claims> = decode(
        &token,
        &state.jwt_decoding,
        &Validation::default(),
    )?;

    Ok(claims.claims)
}

pub async fn init_db_pool(db_url: &str) -> Result_db<PgPool> {
    let pool: PgPool = PgPoolOptions::new()
        .max_connections(20)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .connect(db_url).await?;

    Ok(pool)
}