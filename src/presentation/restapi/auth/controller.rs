use axum::{Extension, Json, extract::State, response::IntoResponse};
use validator::Validate;

use crate::{
    application::{
        auth::{
            dto::{AuthResponse, LoginRequest, RefreshTokenRequest, RegisterRequest},
            error::AuthError,
        },
        user::dto::UserResponse,
    },
    infrastructure::security::jwt::JwtClaims,
    presentation::restapi::{
        auth::router::AuthState,
        response::{ApiResponse, Empty},
    },
};

#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "Registration successful", body = ApiResponse<Empty>),
        (status = 409, description = "User already registered", body = ApiResponse<Empty>),
        (status = 422, description = "Validation error", body = ApiResponse<Empty>),
        (status = 500, description = "Internal server error", body = ApiResponse<Empty>),
    ),
    tag = "auth"
)]
#[axum::debug_handler]
pub async fn register(
    State(state): State<AuthState>,
    Json(dto): Json<RegisterRequest>,
) -> impl IntoResponse {
    if let Err(err) = dto.validate() {
        tracing::error!("failed validate request : {}", err.to_string());
        return ApiResponse::<Empty>::unprocessable_entity(err.to_string());
    }

    match state.auth_usecase.register(dto).await {
        Ok(_) => ApiResponse::success(None),
        Err(AuthError::Conflict) => ApiResponse::conflict("User already registed"),
        Err(_) => ApiResponse::general_error(),
    }
}

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = ApiResponse<AuthResponse>),
        (status = 401, description = "Invalid credentials", body = ApiResponse<Empty>),
        (status = 422, description = "Validation error", body = ApiResponse<Empty>),
        (status = 500, description = "Internal server error", body = ApiResponse<Empty>)
    ),
    tag = "auth",
)]
#[axum::debug_handler]
pub async fn login_with_email(
    State(state): State<AuthState>,
    Json(dto): Json<LoginRequest>,
) -> impl IntoResponse {
    if let Err(err) = dto.validate() {
        return ApiResponse::unprocessable_entity(err.to_string());
    }

    match state.auth_usecase.login(dto).await {
        Ok(data) => ApiResponse::<AuthResponse>::success(Some(data)),
        Err(AuthError::InvalidCredentials) => ApiResponse::unauthorized("password missmatch"),
        Err(_) => ApiResponse::general_error(),
    }
}

#[utoipa::path(
    post,
    path = "/auth/refresh",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Successfully refreshed access token", body = ApiResponse<Option<AuthResponse>>),
        (status = 401, description = "Unauthorized - token expired or invalid", body = ApiResponse<Empty>),
        (status = 422, description = "Unprocessable entity - validation error", body = ApiResponse<Empty>),
        (status = 500, description = "Internal server error", body = ApiResponse<Empty>)
    ),
    tag = "auth",
)]
#[axum::debug_handler]
pub async fn refresh_access_token(
    State(state): State<AuthState>,
    Json(dto): Json<RefreshTokenRequest>,
) -> impl IntoResponse {
    if let Err(err) = dto.validate() {
        return ApiResponse::unprocessable_entity(err.to_string());
    }

    match state.auth_usecase.refresh_access_token(dto).await {
        Ok(data) => ApiResponse::<AuthResponse>::success(Some(data)),
        Err(err) => match err {
            AuthError::TokenExpired | AuthError::NotFound => {
                ApiResponse::unauthorized("Token expired")
            }
            _ => ApiResponse::general_error(),
        },
    }
}

#[utoipa::path(
    get,
    path = "/users/me",
    responses(
        (status = 200, description = "Returns authenticated user information", body = ApiResponse<UserResponse>),
        (status = 401, description = "Unauthorized - invalid JWT claims", body = ApiResponse<Empty>),
        (status = 404, description = "User not found in database", body = ApiResponse<Empty>),
    ),
    tag = "auth",
    security(("bearer_auth" = []))
)]
#[axum::debug_handler]
pub async fn whoami(
    State(state): State<AuthState>,
    Extension(claims): Extension<JwtClaims>,
) -> impl IntoResponse {
    let Ok(data) = state.auth_usecase.whoami(claims.sub).await else {
        return ApiResponse::not_found("User not found");
    };

    ApiResponse::<UserResponse>::success(Some(data))
}

#[utoipa::path(
    delete,
    path = "/auth/logout",
    responses(
        (status = 204, description = "Successfully logged out", body = ApiResponse<Empty>),
        (status = 401, description = "Unauthorized - token not found or expired", body = ApiResponse<Empty>),
        (status = 500, description = "Internal server error", body = ApiResponse<Empty>)
    ),
    tag = "auth",
    security(("bearer_auth" = []))
)]
#[axum::debug_handler]
pub async fn logout(
    State(state): State<AuthState>,
    Extension(claims): Extension<JwtClaims>,
) -> impl IntoResponse {
    match state.auth_usecase.logout(claims.sub).await {
        Ok(()) => ApiResponse::<Empty>::success(None),
        Err(err) => match err {
            AuthError::NotFound => ApiResponse::unauthorized("Token already expired"),
            _ => ApiResponse::general_error(),
        },
    }
}
