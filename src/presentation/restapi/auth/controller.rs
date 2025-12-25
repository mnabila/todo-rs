use axum::{
    Json,
    extract::{Request, State},
    response::IntoResponse,
};
use validator::Validate;

use crate::{
    application::auth::{
        dto::{LoginDto, RefreshTokenDto, RegisterDto},
        error::AuthError,
    },
    infrastructure::security::jwt::JwtClaims,
    presentation::restapi::{auth::router::AuthState, response::ApiResponse},
};

#[axum::debug_handler]
pub async fn register(
    State(state): State<AuthState>,
    Json(dto): Json<RegisterDto>,
) -> impl IntoResponse {
    if let Err(err) = dto.validate() {
        tracing::error!("failed validate request : {}", err);
        return ApiResponse::UnprocessableEntity(err.to_string());
    }

    match state.auth.register(dto).await {
        Ok(()) => ApiResponse::Success(()),
        Err(AuthError::Conflict) => ApiResponse::Conflict("Email already registerd".to_string()),
        Err(_) => ApiResponse::InternalServerError,
    }
}

#[axum::debug_handler]
pub async fn login(State(state): State<AuthState>, Json(dto): Json<LoginDto>) -> impl IntoResponse {
    if let Err(err) = dto.validate() {
        return ApiResponse::UnprocessableEntity(err.to_string());
    }

    match state.auth.login(dto).await {
        Ok(data) => ApiResponse::Success(data),
        Err(AuthError::PasswordMissmatch) => {
            ApiResponse::Unauthorized("Password missmatch".to_string())
        }
        Err(_) => ApiResponse::InternalServerError,
    }
}

#[axum::debug_handler]
pub async fn refresh_access_token(
    State(state): State<AuthState>,
    Json(dto): Json<RefreshTokenDto>,
) -> impl IntoResponse {
    if let Err(err) = dto.validate() {
        return ApiResponse::UnprocessableEntity(err.to_string());
    }

    match state.auth.refresh_access_token(dto).await {
        Ok(data) => ApiResponse::Success(data),
        Err(err) => match err {
            AuthError::TokenExpired | AuthError::NotFound => {
                ApiResponse::Unauthorized("Token expired".to_string())
            }
            _ => ApiResponse::InternalServerError,
        },
    }
}

#[axum::debug_handler]
pub async fn whoami(State(state): State<AuthState>, req: Request) -> impl IntoResponse {
    let Some(claims) = req.extensions().get::<JwtClaims>() else {
        return ApiResponse::NotFound;
    };

    let Ok(data) = state.auth.whoami(claims.sub).await else {
        return ApiResponse::NotFound;
    };

    ApiResponse::Success(data)
}

#[axum::debug_handler]
pub async fn logout(State(state): State<AuthState>, req: Request) -> impl IntoResponse {
    let Some(claims) = req.extensions().get::<JwtClaims>() else {
        return ApiResponse::NotFound;
    };

    match state.auth.logout(claims.sub).await {
        Ok(()) => ApiResponse::Success(()),
        Err(err) => match err {
            AuthError::NotFound => ApiResponse::Unauthorized("Token is expired".to_string()),
            _ => ApiResponse::InternalServerError,
        },
    }
}
