use uuid::Uuid;

use crate::{
    application::user::{dto::UserResponse, error::UserError},
    domain::user::repository::UserRepository,
};

pub struct UserUseCase<T: UserRepository + Send + Sync> {
    user: T,
}

impl<T: UserRepository> UserUseCase<T> {
    pub fn new(user: T) -> Self {
        Self { user }
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<(), UserError> {
        let user = self
            .user
            .find_by_id(id)
            .await
            .map_err(UserError::from)?
            .ok_or(UserError::NotFound)?;

        self.user.delete(user.id).await.map_err(UserError::from)
    }

    pub async fn find_all(&self) -> Result<Vec<UserResponse>, UserError> {
        self.user
            .find_all()
            .await
            .map_err(UserError::from)
            .map(|users| {
                users
                    .into_iter()
                    .map(UserResponse::from)
                    .collect::<Vec<UserResponse>>()
            })
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<UserResponse>, UserError> {
        self.user
            .find_by_id(id)
            .await
            .map_err(UserError::from)
            .map(|user| user.map(UserResponse::from))
    }
}
