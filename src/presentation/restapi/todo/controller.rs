use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    application::todo::{
        dto::{CreateTodoDto, UpdateTodoDto},
        error::TodoError,
    },
    presentation::restapi::{response::ApiResponse, todo::router::TodoState},
};

#[axum::debug_handler]
pub async fn create_todo(
    State(state): State<TodoState>,
    Json(dto): Json<CreateTodoDto>,
) -> impl IntoResponse {
    if let Err(err) = dto.validate() {
        return ApiResponse::UnprocessableEntity(err.to_string());
    }

    match state.todo.create_todo(dto).await {
        Ok(_) => ApiResponse::Success(Option::<()>::None),
        Err(_) => ApiResponse::InternalServerError,
    }
}

#[axum::debug_handler]
pub async fn update_todo(
    State(state): State<TodoState>,
    Path(id): Path<Uuid>,
    Json(dto): Json<UpdateTodoDto>,
) -> impl IntoResponse {
    if let Err(err) = dto.validate() {
        return ApiResponse::UnprocessableEntity(err.to_string());
    }

    match state.todo.update_todo(id, dto).await {
        Ok(todo) => ApiResponse::Success(todo),
        Err(_) => ApiResponse::InternalServerError,
    }
}

#[axum::debug_handler]
pub async fn delete_todo(
    State(state): State<TodoState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.todo.delete_todo(id).await {
        Ok(_) => ApiResponse::Success(Option::<()>::None),
        Err(TodoError::NotFound) => ApiResponse::NotFound,
        Err(_) => ApiResponse::InternalServerError,
    }
}

#[axum::debug_handler]
pub async fn find_all_todo(State(state): State<TodoState>) -> impl IntoResponse {
    match state.todo.find_all().await {
        Ok(todos) => ApiResponse::Success(todos),
        Err(TodoError::NotFound) => ApiResponse::NotFound,
        Err(_) => ApiResponse::InternalServerError,
    }
}

#[axum::debug_handler]
pub async fn find_todo_by_id(
    State(state): State<TodoState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.todo.find_by_id(id).await {
        Ok(todo) => ApiResponse::Success(todo),
        Err(TodoError::NotFound) => ApiResponse::NotFound,
        Err(_) => ApiResponse::InternalServerError,
    }
}
