use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::todo::{error::TodoError, model::Todo};

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn create(&self, todo: &Todo) -> Result<(), TodoError>;
    async fn update(&self, todo: &Todo) -> Result<(), TodoError>;
    async fn delete(&self, id: Uuid) -> Result<(), TodoError>;
    async fn find_all(&self) -> Result<Vec<Todo>, TodoError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Todo>, TodoError>;
}
