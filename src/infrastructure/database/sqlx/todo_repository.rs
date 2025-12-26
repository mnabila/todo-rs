use async_trait::async_trait;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    shared::error::ModelError,
    todo::{model::Todo, repository::TodoRepository},
};

pub struct PostgresTodoRepository {
    pub pool: PgPool,
}

impl PostgresTodoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TodoRepository for PostgresTodoRepository {
    async fn create(&self, todo: &Todo) -> Result<(), ModelError> {
        let _ = sqlx::query("insert into todos(id, user_id, title, description, is_completed, created_at, updated_at) values($1,$2,$3,$4,$5,$6,$7)")
                .bind(todo.id)
                .bind(todo.user_id)
                .bind(todo.title.as_str())
                .bind(todo.description.as_str())
                .bind(todo.is_completed)
                .bind(todo.created_at)
                .bind(todo.updated_at)
                .execute(&self.pool)
                .await
                .map_err(|err| {
                    tracing::error!("todo_repository.find_all : {}", err.to_string());
                    ModelError::Database(err.to_string())
                })?;

        Ok(())
    }

    async fn update(&self, id: Uuid, title: String, description: String) -> Result<(), ModelError> {
        let rows =
            sqlx::query("update todos set title=$1, description=$2, updated_at=$3 where id=$4")
                .bind(title)
                .bind(description)
                .bind(Utc::now())
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|err| {
                    tracing::error!("todo_repository.find_all : {}", err.to_string());
                    ModelError::Database(err.to_string())
                })?
                .rows_affected();

        if rows == 0 {
            return Err(ModelError::NotFound);
        }

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), ModelError> {
        let rows = sqlx::query("delete from todos where id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|err| {
                tracing::error!("todo_repository.delete : {}", err.to_string());
                ModelError::Database(err.to_string())
            })?
            .rows_affected();

        if rows == 0 {
            return Err(ModelError::NotFound);
        }

        Ok(())
    }

    async fn toggle(&self, user_id: Uuid, id: Uuid) -> Result<(), ModelError> {
        let rows = sqlx::query(
            "update todos set is_completed = not is_completed where user_id=$1 and id = $2",
        )
        .bind(user_id)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|err| {
            tracing::error!("todo_repository.toggle : {}", err.to_string());
            ModelError::Database(err.to_string())
        })?
        .rows_affected();

        if rows == 0 {
            return Err(ModelError::NotFound);
        }

        Ok(())
    }

    async fn find_all(&self, user_id: Uuid) -> Result<Vec<Todo>, ModelError> {
        let results = sqlx::query_as::<_, Todo>(
            "select id, user_id, title, description, is_completed, created_at, updated_at from todos where user_id=$1",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|err| {
            tracing::error!("todo_repository.find_all : {}", err.to_string());
            ModelError::NotFound
        })?;

        Ok(results)
    }

    async fn find_by_id(&self, user_id: Uuid, id: Uuid) -> Result<Option<Todo>, ModelError> {
        let result = sqlx::query_as::<_, Todo>(
            "select id, title, description, is_completed, created_at, updated_at from todos where id = $1 and user_id=$2",
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| {
            tracing::error!("todo_repository.find_by_id : {}", err.to_string());
            ModelError::NotFound
        })?;

        match result {
            Some(todo) => Ok(Some(todo)),
            None => Err(ModelError::NotFound),
        }
    }
}
