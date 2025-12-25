use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{shared::error::ModelError, user::model::User};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: User) -> Result<(), ModelError>;
    async fn update(&self, user: &User) -> Result<(), ModelError>;
    async fn delete(&self, id: Uuid) -> Result<(), ModelError>;
    async fn find_all(&self) -> Result<Vec<User>, ModelError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, ModelError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, ModelError>;
    async fn find_by_token(&self, token: &str) -> Result<Option<User>, ModelError>;
}
