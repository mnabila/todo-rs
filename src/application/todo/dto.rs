use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::todo::model::Todo;

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

#[derive(Debug, Serialize)]
pub struct TodoResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Todo> for TodoResponse {
    fn from(value: Todo) -> Self {
        Self {
            id: value.id,
            title: value.title,
            description: value.description,
            is_completed: value.is_completed,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
