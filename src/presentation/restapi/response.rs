use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct Empty {}

#[derive(Serialize, ToSchema)]
pub struct ApiResponse<T>
where
    T: ToSchema,
{
    pub code: String,
    pub message: String,
    pub data: Option<T>,

    #[serde(skip)]
    #[schema(ignore)]
    pub status: StatusCode,
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: ToSchema + Serialize,
{
    fn into_response(self) -> Response {
        (self.status, Json(self)).into_response()
    }
}

impl<T> ApiResponse<T>
where
    T: ToSchema,
{
    pub fn success(data: Option<T>) -> Self {
        Self {
            code: "20000".to_string(),
            message: "success".to_string(),
            data,
            status: StatusCode::OK,
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self {
            code: "40100".to_string(),
            message: message.into(),
            data: None,
            status: StatusCode::UNAUTHORIZED,
        }
    }

    pub fn unprocessable_entity(message: impl Into<String>) -> Self {
        Self {
            code: "42200".to_string(),
            message: message.into(),
            data: None,
            status: StatusCode::UNPROCESSABLE_ENTITY,
        }
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self {
            code: "40900".to_string(),
            message: message.into(),
            data: None,
            status: StatusCode::CONFLICT,
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self {
            code: "40400".to_string(),
            message: message.into(),
            data: None,
            status: StatusCode::NOT_FOUND,
        }
    }

    pub fn general_error() -> Self {
        Self {
            code: "50000".to_string(),
            message: "general error".to_string(),
            data: None,
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
