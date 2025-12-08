use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{domain::shared::error::ModelError, presentation::http::{response::ApiResponse, user::router::UserState}};

#[axum::debug_handler]
pub async fn delete_user(
    State(state): State<UserState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.user.delete_user(id).await {
        Ok(user) => ApiResponse::Success(user),
        Err(ModelError::DeleteError(_, _)) => ApiResponse::InternalServerError,
        Err(_) => ApiResponse::InternalServerError,
    }
}

#[axum::debug_handler]
pub async fn find_all_user(State(state): State<UserState>) -> impl IntoResponse {
    match state.user.find_all().await {
        Ok(users) => ApiResponse::Success(users),
        Err(ModelError::NotFound(_)) => ApiResponse::NotFound,
        Err(_) => ApiResponse::InternalServerError,
    }
}

#[axum::debug_handler]
pub async fn find_user_by_id(
    State(state): State<UserState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.user.find_by_id(id).await {
        Ok(user) => ApiResponse::Success(user),
        Err(ModelError::NotFound(_)) => ApiResponse::NotFound,
        Err(_) => ApiResponse::InternalServerError,
    }
}
