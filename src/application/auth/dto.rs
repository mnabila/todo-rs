use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginDto {
    #[serde(default)]
    #[validate(email, length(min = 1, message = "email is required"))]
    #[schema(example="demo@demo.com")]
    pub email: String,

    #[serde(default)]
    #[validate(length(min = 1, message = "password is required"))]
    #[schema(example="password")]
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub id: Uuid,
    pub name: String,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RefreshTokenDto {
    #[serde(default)]
    #[validate(length(min = 1, message = "token is required"))]
    pub token: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterDto {
    #[serde(default)]
    #[validate(length(min = 1, message = "name is required"))]
    pub name: String,

    #[serde(default)]
    #[validate(
        email(message = "Please enter a valid email address"),
        length(min = 1, message = "email is required")
    )]
    pub email: String,

    #[serde(default)]
    #[validate(length(min = 1, message = "password is required"))]
    pub password: String,
}
