use axum::{
    Extension, Json,
    extract::{Path, State},
    response::IntoResponse,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    application::todo::{
        dto::{CreateTodoDto, TodoResponse, UpdateTodoDto},
        error::TodoError,
    },
    infrastructure::security::jwt::JwtClaims,
    presentation::restapi::{
        response::{ApiResponse, Empty},
        todo::router::TodoState,
    },
};

#[utoipa::path(
    post,
    path = "/todos",
    request_body = CreateTodoDto,
    responses(
        (status = 200, description = "Todo created successfully", body = ApiResponse<Empty>),
        (status = 422, description = "Validation error in request body", body = ApiResponse<Empty>),
        (status = 500, description = "Internal server error", body = ApiResponse<Empty>)
    ),
    tag = "todos",
    security(("bearer_auth" = []))
)]
#[axum::debug_handler]
pub async fn create_todo(
    State(state): State<TodoState>,
    Extension(claims): Extension<JwtClaims>,
    Json(dto): Json<CreateTodoDto>,
) -> impl IntoResponse {
    if let Err(err) = dto.validate() {
        return ApiResponse::<Empty>::unprocessable_entity(err.to_string());
    }

    match state.todo.create_todo(claims.sub, dto).await {
        Ok(_) => ApiResponse::success(None),
        Err(_) => ApiResponse::general_error(),
    }
}

#[utoipa::path(
    put,
    path = "/todos/{id}",
    params(
        ("id"=Uuid, Path, description = "Unique identifier for the todo item"),
    ),
    request_body = UpdateTodoDto,
    responses(
        (status = 200, description = "Todo updated successfully", body = ApiResponse<Empty>),
        (status = 400, description = "Validation error in request body", body = ApiResponse<Empty>),
        (status = 401, description = "Unauthorized - invalid JWT claims", body = ApiResponse<Empty>),
        (status = 404, description = "Todo not found", body = ApiResponse<Empty>),
        (status = 409, description = "Conflict - todo already exists with this ID", body = ApiResponse<Empty>),
        (status = 500, description = "Internal server error", body = ApiResponse<Empty>),
    ),
    tag = "todos",
    security(("bearer_auth" = []))
)]
#[axum::debug_handler]
pub async fn update_todo(
    State(state): State<TodoState>,
    Extension(claims): Extension<JwtClaims>,
    Path(id): Path<Uuid>,
    Json(dto): Json<UpdateTodoDto>,
) -> impl IntoResponse {
    if let Err(err) = dto.validate() {
        return ApiResponse::unprocessable_entity(err.to_string());
    }

    match state.todo.update_todo(claims.sub, id, dto).await {
        Ok(_) => ApiResponse::<Empty>::success(None),
        Err(_) => ApiResponse::general_error(),
    }
}

#[utoipa::path(
    delete,
    path = "/todos/{id}",
    params(
        ("id" = Uuid, Path, description = "Unique identifier for the todo item to be deleted"),
    ),
    responses(
        (status = 200, description = "Todo deleted successfully", body = ApiResponse<Empty>),
        (status = 401, description = "Unauthorized - invalid JWT claims", body = ApiResponse<Empty>),
        (status = 404, description = "Todo not found", body = ApiResponse<Empty>),
        (status = 500, description = "Internal server error", body = ApiResponse<Empty>),
    ),
    tag = "todos",
    security(("bearer_auth" = []))
)]
#[axum::debug_handler]
pub async fn delete_todo(
    State(state): State<TodoState>,
    Extension(claims): Extension<JwtClaims>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.todo.delete_todo(claims.sub, id).await {
        Ok(_) => ApiResponse::<Empty>::success(None),
        Err(TodoError::NotFound) => ApiResponse::not_found("todo not found"),
        Err(_) => ApiResponse::general_error(),
    }
}

#[utoipa::path(
    get,
    path = "/todos",
    responses(
        (status = 200, description = "List of todos retrieved successfully", body = ApiResponse<Vec<TodoResponse>>),
        (status = 401, description = "Unauthorized - invalid JWT claims", body = ApiResponse<Empty>),
        (status = 404, description = "No todos found for the user", body = ApiResponse<Empty>),
        (status = 500, description = "Internal server error", body = ApiResponse<Empty>),
    ),
    tag = "todos",
    security(("bearer_auth" = []))
)]
#[axum::debug_handler]
pub async fn find_all_todo(
    State(state): State<TodoState>,
    Extension(claims): Extension<JwtClaims>,
) -> impl IntoResponse {
    match state.todo.find_all(claims.sub).await {
        Ok(todos) => ApiResponse::<Vec<TodoResponse>>::success(Some(todos)),
        Err(TodoError::NotFound) => ApiResponse::not_found("todo not found"),
        Err(_) => ApiResponse::general_error(),
    }
}

#[utoipa::path(
    get,
    path = "/todos/{id}",
    params(
        ("id" = Uuid, Path, description = "Unique identifier for the todo item"),
    ),
    responses(
        (status = 200, description = "Todo retrieved successfully", body = ApiResponse<TodoResponse>),
        (status = 401, description = "Unauthorized - invalid JWT claims", body = ApiResponse<Empty>),
        (status = 404, description = "Todo not found", body = ApiResponse<Empty>),
        (status = 500, description = "Internal server error", body = ApiResponse<Empty>),
    ),
    tag = "todos",
    security(("bearer_auth" = []))
)]
#[axum::debug_handler]
pub async fn find_todo_by_id(
    State(state): State<TodoState>,
    Extension(claims): Extension<JwtClaims>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.todo.find_by_id(claims.sub, id).await {
        Ok(todo) => ApiResponse::<TodoResponse>::success(todo),
        Err(TodoError::NotFound) => ApiResponse::not_found("todo not_found"),
        Err(_) => ApiResponse::general_error(),
    }
}

#[utoipa::path(
    patch,
    path = "/todos/{id}/toggle",
    params(
        ("id" = Uuid, Path, description = "Unique identifier for the todo item to toggle"),
    ),
    responses(
        (status = 200, description = "Todo toggled successfully", body = ApiResponse<Empty>),
        (status = 401, description = "Unauthorized - invalid JWT claims", body = ApiResponse<Empty>),
        (status = 404, description = "Todo not found", body = ApiResponse<Empty>),
        (status = 500, description = "Internal server error", body = ApiResponse<Empty>),
    ),
    tag = "todos",
    security(("bearer_auth" = []))
)]
#[axum::debug_handler]
pub async fn toggle_todo(
    State(state): State<TodoState>,
    Extension(claims): Extension<JwtClaims>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.todo.toggle_todo(claims.sub, id).await {
        Ok(_) => ApiResponse::<Empty>::success(None),
        Err(TodoError::NotFound) => ApiResponse::not_found("todo not found"),
        Err(_) => ApiResponse::general_error(),
    }
}
