use uuid::Uuid;

use crate::domain::{
    shared::error::ModelError,
    user::{model::User, repository::UserRepository},
};

pub struct UserUseCase<T: UserRepository + Send + Sync> {
    user: T,
}

impl<T: UserRepository> UserUseCase<T> {
    pub fn new(user: T) -> Self {
        Self { user }
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<(), ModelError> {
        self.user.delete(id).await
    }

    pub async fn find_all(&self) -> Result<Vec<User>, ModelError> {
        self.user.find_all().await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, ModelError> {
        self.user.find_by_id(id).await
    }
}
