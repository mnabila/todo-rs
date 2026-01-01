use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::{
    application::{
        auth::{
            dto::{AuthResponse, LoginDto, RefreshTokenDto, RegisterDto},
            error::AuthError,
        },
        user::dto::UserResponse,
    },
    domain::user::{model::User, repository::UserRepository},
    infrastructure::security::{jwt::JwtClaims, token::Token},
};

pub struct AuthUseCase<T: UserRepository + Send + Sync> {
    user: T,
    jwt_secret: String,
    jwt_duration: Duration,
}

impl<T: UserRepository> AuthUseCase<T> {
    pub fn new(user: T, jwt_secret: String, jwt_duration: i64) -> Self {
        Self {
            user,
            jwt_secret,
            jwt_duration: Duration::minutes(jwt_duration),
        }
    }

    pub async fn register(&self, dto: RegisterDto) -> Result<User, AuthError> {
        let password = User::hash_password(dto.password.as_str()).map_err(|err| {
            tracing::error!("failed hash password : {}", err);
            AuthError::GeneralError
        })?;

        let user = User {
            id: Uuid::new_v4(),
            name: dto.name,
            email: dto.email.to_lowercase(),
            password,
            token: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.user.create(user).await.map_err(AuthError::from)
    }

    pub async fn login(&self, dto: LoginDto) -> Result<AuthResponse, AuthError> {
        let mut user = match self
            .user
            .find_by_email(&dto.email)
            .await
            .map_err(AuthError::from)?
        {
            Some(u) => u,
            None => return Err(AuthError::NotFound),
        };

        if !user.verify_password(dto.password) {
            return Err(AuthError::InvalidCredentials);
        }

        let refresh_token = Token::new(&self.jwt_secret, &Uuid::new_v4().to_string())
            .map_err(|_| AuthError::GeneralError)?;

        user.token = Some(refresh_token.encrypted);
        user.updated_at = Utc::now();

        self.user
            .update(&user)
            .await
            .map_err(|_| AuthError::GeneralError)?;

        let access_token = JwtClaims::new(user.id, self.jwt_duration)
            .encode(&self.jwt_secret)
            .map_err(|_| AuthError::GeneralError)?;

        Ok(AuthResponse {
            id: user.id,
            name: user.name,
            access_token,
            refresh_token: refresh_token.decrypted,
        })
    }

    pub async fn refresh_access_token(
        &self,
        dto: RefreshTokenDto,
    ) -> Result<AuthResponse, AuthError> {
        let refresh_token =
            Token::new(&self.jwt_secret, &dto.token).map_err(|_| AuthError::GeneralError)?;

        let user = self
            .user
            .find_by_token(&refresh_token.encrypted)
            .await?
            .ok_or(AuthError::NotFound)?;

        let Some(token) = user.token else {
            tracing::error!("token is empty");
            return Err(AuthError::GeneralError);
        };

        if !refresh_token.validate(&token) {
            return Err(AuthError::TokenExpired);
        }

        let access_token = JwtClaims::new(user.id, self.jwt_duration)
            .encode(&self.jwt_secret)
            .map_err(|err| {
                tracing::error!("unable generate access token : {}", err);
                AuthError::GeneralError
            })?;

        Ok(AuthResponse {
            id: user.id,
            name: user.name,
            access_token,
            refresh_token: refresh_token.decrypted,
        })
    }

    pub async fn whoami(&self, id: Uuid) -> Result<UserResponse, AuthError> {
        let user = self
            .user
            .find_by_id(id)
            .await?
            .map(UserResponse::from)
            .ok_or(AuthError::NotFound)?;

        Ok(user)
    }

    pub async fn logout(&self, id: Uuid) -> Result<(), AuthError> {
        let mut user = match self.user.find_by_id(id).await.map_err(AuthError::from)? {
            Some(u) => u,
            None => return Err(AuthError::NotFound),
        };

        user.token = None;
        user.updated_at = Utc::now();

        self.user
            .update(&user)
            .await
            .map(|_| ())
            .map_err(|_| AuthError::GeneralError)
    }
}

#[cfg(test)]
mod tests {

    use chrono::Utc;
    use mockall::predicate::eq;
    use uuid::Uuid;

    use crate::{
        application::auth::{
            dto::{LoginDto, RefreshTokenDto, RegisterDto},
            error::AuthError,
            usecase::AuthUseCase,
        },
        domain::{
            shared::error::ModelError,
            user::{model::User, repository::MockUserRepository},
        },
        infrastructure::security::token::Token,
    };

    const JWT_SECRET: &str = "secret";
    const JWT_DURATION: i64 = 10;

    #[tokio::test]
    async fn register_success() {
        let mut repo = MockUserRepository::new();

        repo.expect_create().return_once(|user| {
            Box::pin(async move {
                if user.email.is_empty() {
                    Err(ModelError::NotFound)
                } else {
                    Ok(user)
                }
            })
        });

        let auth = AuthUseCase::new(repo, JWT_SECRET.to_string(), JWT_DURATION);

        let dto = RegisterDto {
            name: "test".to_string(),
            email: "test@domain.com".to_string(),
            password: "test123".to_string(),
        };

        let result = auth.register(dto).await;

        if let Ok(user) = result {
            assert!(!user.name.is_empty(), "name should not be empty");
            assert!(!user.email.is_empty(), "email should not be empty");
        }
    }

    #[tokio::test]
    async fn register_duplicated_email() {
        let mut repo = MockUserRepository::new();

        repo.expect_create()
            .withf(|user| user.email == "test@domain.com")
            .return_once(|_| Box::pin(async move { Err(ModelError::Conflict) }));

        let auth = AuthUseCase::new(repo, JWT_SECRET.to_string(), JWT_DURATION);

        let dto = RegisterDto {
            name: "test".to_string(),
            email: "test@domain.com".to_string(),
            password: "test123".to_string(),
        };

        let result = auth.register(dto).await;

        assert!(matches!(result.unwrap_err(), AuthError::Conflict));
    }

    #[tokio::test]
    async fn login_success() {
        let mut repo = MockUserRepository::new();

        repo.expect_find_by_email().return_once(|_| {
            Box::pin(async {
                let password = User::hash_password("test123").unwrap();
                let user = User {
                    id: Uuid::new_v4(),
                    name: "test".to_string(),
                    email: "test@domain.com".to_string(),
                    password,
                    token: Some(Uuid::new_v4().to_string()),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                };
                Ok(Some(user))
            })
        });

        repo.expect_update().times(1).returning(|user| {
            let user_owned = user.clone();
            Box::pin(async move { Ok(user_owned) })
        });

        let auth = AuthUseCase::new(repo, JWT_SECRET.to_string(), JWT_DURATION);

        let dto = LoginDto {
            email: "test@domain.com".to_string(),
            password: "test123".to_string(),
        };

        let result = auth.login(dto).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn login_failure_wrong_password() {
        let mut repo = MockUserRepository::new();

        repo.expect_find_by_email().return_once(|_| {
            Box::pin(async {
                let password = User::hash_password("demo123").unwrap();
                let user = User {
                    id: Uuid::new_v4(),
                    name: "test".to_string(),
                    email: "test@domain.com".to_string(),
                    password,
                    token: None,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                };
                Ok(Some(user))
            })
        });

        let auth = AuthUseCase::new(repo, JWT_SECRET.to_string(), JWT_DURATION);

        let dto = LoginDto {
            email: "test@domain.com".to_string(),
            password: "test123".to_string(),
        };

        let result = auth.login(dto).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn refresh_access_token_success() {
        let token = Token::new(&JWT_SECRET.to_string(), &Uuid::new_v4().to_string()).unwrap();

        let mut repo = MockUserRepository::new();

        repo.expect_find_by_token()
            .with(eq(token.encrypted.clone()))
            .return_once(move |_| {
                let encripted = token.encrypted.clone();
                Box::pin(async {
                    let password = User::hash_password("demo123").unwrap();
                    let user = User {
                        id: Uuid::new_v4(),
                        name: "test".to_string(),
                        email: "test@domain.com".to_string(),
                        password,
                        token: Some(encripted),
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                    };

                    Ok(Some(user))
                })
            });

        let auth = AuthUseCase::new(repo, JWT_SECRET.to_string(), JWT_DURATION);

        let dto = RefreshTokenDto {
            token: token.decrypted,
        };

        let result = auth.refresh_access_token(dto).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn whoami_success() {
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

        let auth = AuthUseCase::new(repo, JWT_SECRET.to_string(), JWT_DURATION);
        assert!(auth.whoami(id).await.is_ok());
    }

    #[tokio::test]
    async fn whoami_not_found() {
        let id = Uuid::new_v4();
        let mut repo = MockUserRepository::new();

        repo.expect_find_by_id()
            .withf(move |user_id| *user_id == id)
            .return_once(move |_| Box::pin(async move { Ok(None) }));

        let auth = AuthUseCase::new(repo, JWT_SECRET.to_string(), JWT_DURATION);
        assert!(auth.whoami(id).await.is_err());
    }

    #[tokio::test]
    async fn refresh_access_token_failed() {
        let token = Token::new(&JWT_SECRET.to_string(), &Uuid::new_v4().to_string()).unwrap();

        let mut repo = MockUserRepository::new();

        repo.expect_find_by_token()
            .with(eq(token.encrypted.clone()))
            .return_once(move |_| {
                Box::pin(async {
                    let password = User::hash_password("demo123").unwrap();
                    let user = User {
                        id: Uuid::new_v4(),
                        name: "test".to_string(),
                        email: "test@domain.com".to_string(),
                        password,
                        token: None,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                    };

                    Ok(Some(user))
                })
            });

        let auth = AuthUseCase::new(repo, JWT_SECRET.to_string(), JWT_DURATION);

        let dto = RefreshTokenDto {
            token: token.decrypted,
        };

        let result = auth.refresh_access_token(dto).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn logout_success() {
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

        repo.expect_update()
            .withf(|u| u.token.is_none())
            .return_once(|u| {
                let user = u.clone();
                Box::pin(async move { Ok(user) })
            });

        let auth = AuthUseCase::new(repo, JWT_SECRET.to_string(), JWT_DURATION);
        assert!(auth.logout(id).await.is_ok());
    }

    #[tokio::test]
    async fn logout_user_not_found() {
        let id = Uuid::new_v4();
        let mut repo = MockUserRepository::new();

        repo.expect_find_by_id()
            .withf(move |user_id| *user_id == id)
            .return_once(move |_| Box::pin(async move { Ok(None) }));

        repo.expect_update()
            .withf(|u| u.token.is_none())
            .return_once(|u| {
                let user = u.clone();
                Box::pin(async move { Ok(user) })
            });

        let auth = AuthUseCase::new(repo, JWT_SECRET.to_string(), JWT_DURATION);
        assert!(auth.logout(id).await.is_err());
    }
}
