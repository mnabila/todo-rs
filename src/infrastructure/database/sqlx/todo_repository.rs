use async_trait::async_trait;
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
    async fn create(&self, todo: Todo) -> Result<Todo, ModelError> {
        let created = sqlx::query_as::<_, Todo>(
            r#"
            INSERT INTO 
            todos(id, user_id, title, description, is_completed, created_at, updated_at) 
            VALUES ($1,$2,$3,$4,$5,$6,$7)
            RETURNING
            id, user_id, title, description, is_completed, created_at, updated_at
            "#,
        )
        .bind(todo.id)
        .bind(todo.user_id)
        .bind(todo.title.as_str())
        .bind(todo.description.as_str())
        .bind(todo.is_completed)
        .bind(todo.created_at)
        .bind(todo.updated_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| {
            tracing::error!("todo_repository.find_all : {}", err.to_string());
            ModelError::Database(err.to_string())
        })?;

        Ok(created)
    }

    async fn update(&self, todo: Todo) -> Result<Todo, ModelError> {
        let updated = sqlx::query_as::<_, Todo>(
            r#"
            UPDATEtodos
            SET 
            title=$1, description=$2, updated_at=$3 WHERE id=$4",
            RETURNING
            id, user_id, title, description, is_completed, created_at, updated_at
            "#,
        )
        .bind(todo.title)
        .bind(todo.description)
        .bind(todo.updated_at)
        .bind(todo.id)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| {
            tracing::error!("todo_repository.find_all : {}", err.to_string());
            ModelError::Database(err.to_string())
        })?;

        Ok(updated)
    }

    async fn delete(&self, id: Uuid) -> Result<(), ModelError> {
        let rows = sqlx::query("DELETE FROM todos WHERE id = $1")
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
            "UPDATE todos SET is_completed = NOT is_completed WHERE user_id=$1 AND id = $2",
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
            "SELECT id, user_id, title, description, is_completed, created_at, updated_at FROM todos WHERE user_id=$1",
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
            "SELECT id, title, description, is_completed, created_at, updated_at FROM todos WHERE id = $1 AND user_id=$2",
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
