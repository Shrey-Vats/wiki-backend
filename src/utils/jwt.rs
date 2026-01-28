
use chrono::{Duration as DurationC, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode, errors::Result};
use uuid::Uuid;

use crate::state::{Claims};

pub async fn create_jwt_token(user_id: Uuid, encoding_key: EncodingKey) -> Result<String> {
    let now = Utc::now();

    let claims = Claims {
        user_id: user_id,
        iat: now.timestamp() as usize,
        exp: (now + DurationC::days(7)).timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &encoding_key,
    )?;

    Ok(token)
}

pub fn verify_jwt_token(token: &str, decoding_key: DecodingKey) -> Result<Claims> {
    let claims: TokenData<Claims> = decode(
        &token,
        &decoding_key,
        &Validation::default(),
    )?;

    Ok(claims.claims)
}
