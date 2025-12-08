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
    NotFound,
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match self {
            ApiResponse::Success(data) => {
                let status = StatusCode::OK;
                respond(status, status.as_str(), Some(data))
            }
            ApiResponse::UnprocessableEntity(msg) => {
                let status = StatusCode::UNPROCESSABLE_ENTITY;
                respond(status, &msg, Option::<()>::None)
            }
            ApiResponse::InternalServerError => {
                let status = StatusCode::INTERNAL_SERVER_ERROR;
                respond(status, status.as_str(), None::<T>)
            }
            ApiResponse::NotFound => {
                let status = StatusCode::NOT_FOUND;
                respond(status, status.as_str(), None::<T>)
            }
        }
    }
}

fn respond<T>(status: StatusCode, message: &str, data: Option<T>) -> Response
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
