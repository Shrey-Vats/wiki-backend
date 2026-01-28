use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{Response, Result},
};
use axum_extra::extract::CookieJar;
use crate::{modules::user::model::UserId, state::AppState, utils::jwt::verify_jwt_token};

pub async fn auth_middleware(
    State(state): State<AppState>,
    jar: CookieJar,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = jar
        .get("jwt")
        .map(|c| c.value().to_string())
        .ok_or(StatusCode::UNAUTHORIZED)?;


    let token_data = verify_jwt_token(&token, state.jwt_decoding).map_err(|_| StatusCode::UNAUTHORIZED)?;
   
    req.extensions_mut().insert(UserId(token_data.user_id));

    Ok(next.run(req).await)
}
