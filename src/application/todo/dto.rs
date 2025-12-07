use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTodoDto {
    #[serde(default)]
    #[validate(length(min = 1, message = "title is required"))]
    pub title: String,

    #[serde(default)]
    #[validate(length(max = 255))]
    pub description: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTodoDto {
    #[serde(default)]
    #[validate(length(min = 1, message = "title is required"))]
    pub title: String,

    #[serde(default)]
    #[validate(length(max = 255))]
    pub description: String,
}
