use uuid::Uuid;

use crate::{
    application::user::{dto::UserResponse, error::UserError},
    domain::user::repository::UserRepository,
};

pub struct UserUseCase<T: UserRepository + Send + Sync> {
    user_repository: T,
}

impl<T: UserRepository> UserUseCase<T> {
    pub fn new(user: T) -> Self {
        Self {
            user_repository: user,
        }
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<(), UserError> {
        let user = self
            .user_repository
            .find_by_id(id)
            .await
            .map_err(UserError::from)?
            .ok_or(UserError::NotFound)?;

        self.user_repository
            .delete(user.id)
            .await
            .map_err(UserError::from)
    }

    pub async fn find_all(&self) -> Result<Vec<UserResponse>, UserError> {
        self.user_repository
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
        self.user_repository
            .find_by_id(id)
            .await
            .map_err(UserError::from)
            .map(|user| user.map(UserResponse::from))
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;

    use crate::{
        application::user::usecase::UserUseCase,
        domain::{
            shared::error::ModelError,
            user::{model::User, repository::MockUserRepository},
        },
    };

    #[tokio::test]
    async fn delete_user_success() {
        let id = Uuid::new_v4();
        let mut repo = MockUserRepository::new();

        repo.expect_find_by_id()
            .withf(move |user_id| *user_id == id)
            .return_once(move |_| {
                let password = User::hash_password("demo123").unwrap();
                let user = User {
                    id,
                    name: "test".to_string(),
                    email: "test@domain.com".to_string(),
                    password,
                    token: None,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                };

                Box::pin(async move { Ok(Some(user)) })
            });

        repo.expect_delete()
            .return_once(|_| Box::pin(async move { Ok(()) }));

        let usecase = UserUseCase::new(repo);
        assert!(usecase.delete_user(id).await.is_ok());
    }

    #[tokio::test]
    async fn delete_user_failed() {
        let id = Uuid::new_v4();
        let mut repo = MockUserRepository::new();

        repo.expect_find_by_id()
            .withf(move |user_id| *user_id == id)
            .return_once(|_| Box::pin(async { Err(ModelError::NotFound) }));

        repo.expect_delete()
            .return_once(|_| Box::pin(async { Err(ModelError::NotFound) }));

        let usecase = UserUseCase::new(repo);
        assert!(usecase.delete_user(id).await.is_err());
    }

    #[tokio::test]
    async fn find_all_success() {
        let mut repo = MockUserRepository::new();

        repo.expect_find_all().return_once(|| {
            let users = vec![
                User {
                    id: Uuid::new_v4(),
                    name: "Alice".to_string(),
                    email: "alice@example.com".to_string(),
                    password: User::hash_password("demo123").unwrap(),
                    token: None,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                },
                User {
                    id: Uuid::new_v4(),
                    name: "Bob".to_string(),
                    email: "bob@example.com".to_string(),
                    password: User::hash_password("demo123").unwrap(),
                    token: None,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                },
                User {
                    id: Uuid::new_v4(),
                    name: "Charlie".to_string(),
                    email: "charlie@example.com".to_string(),
                    password: User::hash_password("demo123").unwrap(),
                    token: None,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                },
            ];

            Box::pin(async { Ok(users) })
        });

        let usecase = UserUseCase::new(repo);
        assert!(usecase.find_all().await.is_ok());
    }

    #[tokio::test]
    async fn find_all_failed() {
        let mut repo = MockUserRepository::new();

        repo.expect_find_all()
            .return_once(|| Box::pin(async { Err(ModelError::NotFound) }));

        let usecase = UserUseCase::new(repo);
        assert!(usecase.find_all().await.is_err());
    }

    #[tokio::test]
    async fn find_by_id_success() {
        let id = Uuid::new_v4();
        let mut repo = MockUserRepository::new();

        repo.expect_find_by_id()
            .withf(move |user_id| *user_id == id)
            .return_once(move |_| {
                let password = User::hash_password("demo123").unwrap();
                let user = User {
                    id,
                    name: "test".to_string(),
                    email: "test@domain.com".to_string(),
                    password,
                    token: None,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                };

                Box::pin(async move { Ok(Some(user)) })
            });

        let usecase = UserUseCase::new(repo);
        assert!(usecase.find_by_id(id).await.is_ok());
    }

    #[tokio::test]
    async fn find_by_id_failed() {
        let id = Uuid::new_v4();
        let mut repo = MockUserRepository::new();

        repo.expect_find_by_id()
            .withf(move |user_id| *user_id == id)
            .return_once(|_| Box::pin(async { Err(ModelError::NotFound) }));

        let usecase = UserUseCase::new(repo);
        assert!(usecase.find_by_id(id).await.is_err());
    }
}
