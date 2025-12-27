use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    application::user::{dto::UserResponse, error::UserError},
    presentation::restapi::{
        response::{ApiResponse, Empty},
        user::router::UserState,
    },
};

#[utoipa::path(
    delete,
    path = "/users/{id}",
    params(
        ("id"=i32, Path, description = "Unique user identifier")
    ),
    responses(
        (status = 200, description = "User deleted successfully", body = ApiResponse<Empty>),
        (status = 404, description = "User not found", body = ApiResponse<Empty>),
        (status = 500, description = "Internal server error", body = ApiResponse<Empty>)
    ),
    tag = "users",
    security(("bearer_auth" = []))
)]
#[axum::debug_handler]
pub async fn delete_user(
    State(state): State<UserState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.user.delete_user(id).await {
        Ok(_) => ApiResponse::<Empty>::success(None),
        Err(UserError::NotFound) => ApiResponse::not_found("User not found"),
        Err(_) => ApiResponse::general_error(),
    }
}

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "Returns a list of all users", body = ApiResponse<Vec<UserResponse>>),
        (status = 404, description = "User not found", body = ApiResponse<Empty>),
        (status = 500, description = "Internal server error", body = ApiResponse<Empty>)
    ),
    tag = "users",
    security(("bearer_auth" = []))
)]
#[axum::debug_handler]
pub async fn find_all_user(State(state): State<UserState>) -> impl IntoResponse {
    match state.user.find_all().await {
        Ok(users) => ApiResponse::<Vec<UserResponse>>::success(Some(users)),
        Err(UserError::NotFound) => ApiResponse::not_found("User not found"),
        Err(_) => ApiResponse::general_error(),
    }
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id"=i32, Path, description = "Unique user identifier")
    ),
    responses(
        (status = 200, description = "User retrieved successfully", body = UserResponse),
        (status = 404, description = "User not found", body = ApiResponse<Empty>),
        (status = 500, description = "Internal server error", body = ApiResponse<Empty>)
    ),
    tag = "users",
    security(("bearer_auth" = []))
)]
#[axum::debug_handler]
pub async fn find_user_by_id(
    State(state): State<UserState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.user.find_by_id(id).await {
        Ok(user) => ApiResponse::<UserResponse>::success(user),
        Err(UserError::NotFound) => ApiResponse::not_found("User not found"),
        Err(_) => ApiResponse::general_error(),
    }
}
