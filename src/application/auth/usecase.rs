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

    pub async fn register(&self, dto: RegisterDto) -> Result<(), AuthError> {
        let password = User::hash_password(dto.password.as_str()).map_err(|err| {
            tracing::error!("failed hash password : {}", err);
            AuthError::GeneralError
        })?;

        let user = User {
            id: Uuid::new_v4(),
            name: dto.name,
            email: dto.email.to_lowercase(),
            password: password,
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
            return Err(AuthError::PasswordMissmatch);
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
            access_token: access_token,
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
            access_token: access_token,
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
    //
}
