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
    async fn delete(&self, id: Uuid) -> Result<(), ModelError> {
        let rows = sqlx::query("delete from users where id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|err| {
                tracing::error!("user_repository.delete : {}", err.to_string());
                ModelError::DeleteError("User", err.to_string())
            })?
            .rows_affected();

        if rows == 0 {
            return Err(ModelError::NotFound("User"));
        }

        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<User>, ModelError> {
        let results =
            sqlx::query_as::<_, User>("select id, name, email, created_at, updated_at from users")
                .fetch_all(&self.pool)
                .await
                .map_err(|err| {
                    tracing::error!("user_repository.find_all : {}", err.to_string());
                    ModelError::NotFound("User")
                })?;

        Ok(results)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, ModelError> {
        let result = sqlx::query_as::<_, User>(
            "select id, name, email, created_at, updated_at from users where id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| {
            tracing::error!("user_repository.find_by_id : {}", err.to_string());
            ModelError::NotFound("User")
        })?;

        match result {
            Some(todo) => Ok(Some(todo)),
            None => Err(ModelError::NotFound("User")),
        }
    }
}
