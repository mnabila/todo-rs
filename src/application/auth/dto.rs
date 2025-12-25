use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginDto {
    #[serde(default)]
    #[validate(email, length(min = 1, message = "email is required"))]
    pub email: String,

    #[serde(default)]
    #[validate(length(min = 1, message = "password is required"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub id: Uuid,
    pub name: String,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RefreshTokenDto {
    #[serde(default)]
    #[validate(length(min = 1, message = "token is required"))]
    pub token: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterDto {
    #[serde(default)]
    #[validate(length(min = 1, message = "name is required"))]
    pub name: String,

    #[serde(default)]
    #[validate(email, length(min = 1, message = "email is required"))]
    pub email: String,

    #[serde(default)]
    #[validate(length(min = 1, message = "password is required"))]
    pub password: String,
}
