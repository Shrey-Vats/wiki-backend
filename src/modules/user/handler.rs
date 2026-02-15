use axum::{Extension, Json, extract::State, response::IntoResponse};
use axum_extra::extract::CookieJar;
use axum_macros::debug_handler;
use tower_cookies::Cookie;

use crate::{
    common::{error::{AppError, ValidationError}, response::ApiResponse},
    modules::user::{
        model::{LoginCredentials, LoginDto, SignUpCredentials, SignUpDto, UserId},
    },
    state::AppState,
    utils::jwt::create_jwt_token,
};

#[debug_handler]
pub async fn create_user(
    State(state): State<AppState>,
    cookies: CookieJar,
    Json(user): Json<SignUpDto>,
) -> Result<impl IntoResponse, AppError> {
    let new_user: SignUpCredentials = user.try_into()?;

    let user = state.user_service.create(new_user).await?;

    let jwt = create_jwt_token(user.id, state.jwt_encoding)
        .await
        .map_err(|_| AppError::Validation(ValidationError::FailedToCreateToken))?;

    let jar = cookies.add(Cookie::build(("jwt", jwt)).http_only(true).path("/"));

    Ok((
        jar,
        Json(ApiResponse::success("User created successfuly", user)),
    ))
}

pub async fn login_user(
    State(state): State<AppState>,
    cookies: CookieJar,
    Json(dto): Json<LoginDto>,
) -> Result<impl IntoResponse, AppError> {
    let new_user: LoginCredentials = dto.try_into()?;

    let user = state.user_service.login(new_user).await?;

    let jwt = create_jwt_token(user.id, state.jwt_encoding)
        .await
        .map_err(|_| AppError::Validation(ValidationError::FailedToCreateToken))?;

    let jar = cookies.add(Cookie::build(("jwt", jwt)).http_only(true).path("/"));

    Ok((
        jar,
        Json(ApiResponse::success("User login successfuly", user)),
    ))
}

pub async fn delete_user_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {

    state.user_service.delete(user_id.0).await?;

    Ok(Json(ApiResponse::success("User deleted successfuly", None::<()>)))
}

pub async fn get_user_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let user = state.user_service.get(user_id.0).await?;

    Ok(Json(ApiResponse::success("fetch user successfuly", user)))
}
