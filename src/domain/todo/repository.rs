use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{shared::error::ModelError, todo::model::Todo};

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn create(&self, todo: &Todo) -> Result<(), ModelError>;
    async fn update(&self, id: Uuid, title: String, description: String) -> Result<(), ModelError>;
    async fn delete(&self, id: Uuid) -> Result<(), ModelError>;
    async fn find_all(&self) -> Result<Vec<Todo>, ModelError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Todo>, ModelError>;
}
