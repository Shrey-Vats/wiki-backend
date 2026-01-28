use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow};
use time::PrimitiveDateTime;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_encoding: EncodingKey,
    pub jwt_decoding: DecodingKey,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub user_id: Uuid,
    // pub role: String,
    pub exp: usize, // expiry timestamp
    pub iat: usize, // current timestamp
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignUpCredentials {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone, Copy, Debug)]
pub struct UserId(pub Uuid);




impl SignUpCredentials {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.name.trim().len() < 3 {
            return Err("Name must be at least 5 characters");
        }

        if self.password.trim().len() < 6 {
            return Err("Password must contains at least 6 cherecters");
        }

        if !self.email.contains("@") {
            return Err("Invalid Email");
        }

        Ok(())
    }
}

impl LoginCredentials {
    fn validate(&self) -> Result<(), &'static str> {
        if self.password.trim().len() < 6 {
            return Err("Password must contains at least 6 cherecters");
        }

        if !self.email.contains("@") {
            return Err("Invalid Email");
        }

        Ok(())
    }
}
