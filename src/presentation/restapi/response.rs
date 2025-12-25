use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiBody<T> {
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

pub enum ApiResponse<T> {
    Success(T),
    UnprocessableEntity(String),
    InternalServerError,
    Conflict(String),
    NotFound,
    Unauthorized(String),
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match self {
            ApiResponse::Success(data) => {
                let status = StatusCode::OK;
                response(
                    status,
                    status.canonical_reason().unwrap_or(status.as_str()),
                    Some(data),
                )
            }
            ApiResponse::UnprocessableEntity(message) => {
                let status = StatusCode::UNPROCESSABLE_ENTITY;
                response(status, &message, Option::<()>::None)
            }
            ApiResponse::InternalServerError => {
                let status = StatusCode::INTERNAL_SERVER_ERROR;
                response(
                    status,
                    status.canonical_reason().unwrap_or(status.as_str()),
                    Option::<()>::None,
                )
            }
            ApiResponse::NotFound => {
                let status = StatusCode::NOT_FOUND;
                response(
                    status,
                    status.canonical_reason().unwrap_or(status.as_str()),
                    Option::<()>::None,
                )
            }
            ApiResponse::Unauthorized(message) => {
                let status = StatusCode::UNAUTHORIZED;
                response(status, &message, Option::<()>::None)
            }
            ApiResponse::Conflict(message) => {
                let status = StatusCode::CONFLICT;
                response(status, &message, Option::<()>::None)
            }
        }
    }
}

fn response<T>(status: StatusCode, message: &str, data: Option<T>) -> Response
where
    T: Serialize,
{
    let body = ApiBody {
        code: status.as_u16(),
        message: message.to_string(),
        data,
    };

    (status, Json(body)).into_response()
}
