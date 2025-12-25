use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    shared::error::ModelError,
    user::{model::User, repository::UserRepository},
};

pub struct PostgresUserRepository {
    pub pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: User) -> Result<(), ModelError> {
        sqlx::query("insert into users(id, name, email, password, created_at, updated_at) values($1,$2,$3,$4,$5,$6)")
            .bind(user.id)
            .bind(user.name)
            .bind(user.email)
            .bind(user.password)
            .bind(user.created_at)
            .bind(user.updated_at)
            .execute(&self.pool)
            .await
            .map(|_|())
            .map_err(|err| {
                tracing::error!("user_repository.create : {}", err.to_string());
                ModelError::Conflict
            })
    }

    async fn update(&self, user: &User) -> Result<(), ModelError> {
        sqlx::query(
            "update users set name=$1, email=$2, password=$3, token=$4, updated_at=$5 where id =$6",
        )
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.password)
        .bind(&user.token)
        .bind(&user.updated_at)
        .bind(&user.id)
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(|err| {
            tracing::error!("user_repository.update : {}", err.to_string());
            ModelError::Database(err.to_string())
        })
    }

    async fn delete(&self, id: Uuid) -> Result<(), ModelError> {
        let rows = sqlx::query("delete from users where id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|err| {
                tracing::error!("user_repository.delete : {}", err.to_string());
                ModelError::Database(err.to_string())
            })?
            .rows_affected();

        if rows == 0 {
            return Err(ModelError::NotFound);
        }

        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<User>, ModelError> {
        let results =
            sqlx::query_as::<_, User>("select id, name, email, password, token, created_at, updated_at from users")
                .fetch_all(&self.pool)
                .await
                .map_err(|err| {
                    tracing::error!("user_repository.find_all : {}", err.to_string());
                    ModelError::NotFound
                })?;

        Ok(results)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, ModelError> {
        let result = sqlx::query_as::<_, User>(
            "select id, name, email, password, token, created_at, updated_at from users where id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| {
            tracing::error!("user_repository.find_by_id : {}", err.to_string());
            ModelError::Database(err.to_string())
        })?;

        match result {
            Some(todo) => Ok(Some(todo)),
            None => Err(ModelError::NotFound),
        }
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, ModelError> {
        let result = sqlx::query_as::<_, User>(
            "select id, name, email, password, token, created_at, updated_at from users where email = $1",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| {
            tracing::error!("user_repository.find_by_email : {}", err.to_string());
            ModelError::Database(err.to_string())
        })?;

        match result {
            Some(todo) => Ok(Some(todo)),
            None => Err(ModelError::NotFound),
        }
    }

    async fn find_by_token(&self, token: &str) -> Result<Option<User>, ModelError> {
        let result = sqlx::query_as::<_, User>(
            "select id, name, email, password, token, created_at, updated_at from users where token = $1",
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| {
            tracing::error!("user_repository.find_by_token : {}", err.to_string());
            ModelError::Database(err.to_string())
        })?;

        match result {
            Some(todo) => Ok(Some(todo)),
            None => Err(ModelError::NotFound),
        }
    }
}
