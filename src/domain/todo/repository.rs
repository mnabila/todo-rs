use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{shared::error::ModelError, todo::model::Todo};

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn create(&self, todo: &Todo) -> Result<(), ModelError>;
    async fn update(&self, id: Uuid, title: String, description: String) -> Result<(), ModelError>;
    async fn delete(&self, id: Uuid) -> Result<(), ModelError>;
    async fn toggle(&self, user_id: Uuid, id: Uuid) -> Result<(), ModelError>;
    async fn find_all(&self, user_id: Uuid) -> Result<Vec<Todo>, ModelError>;
    async fn find_by_id(&self, user_id: Uuid, id: Uuid) -> Result<Option<Todo>, ModelError>;
}
