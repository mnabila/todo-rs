use chrono::Utc;
use uuid::Uuid;

use crate::{
    application::todo::{
        dto::{CreateTodoDto, UpdateTodoDto},
        error::TodoError,
    },
    domain::todo::{model::Todo, repository::TodoRepository},
};

pub struct TodoUseCase<T: TodoRepository + Send + Sync> {
    todo: T,
}

impl<T: TodoRepository> TodoUseCase<T> {
    pub fn new(todo: T) -> Self {
        Self { todo }
    }

    pub async fn create_todo(&self, dto: CreateTodoDto) -> Result<(), TodoError> {
        let todo = Todo {
            id: Uuid::new_v4(),
            title: dto.title,
            description: dto.description,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.todo.create(&todo).await.map_err(TodoError::from)
    }

    pub async fn update_todo(&self, id: Uuid, dto: UpdateTodoDto) -> Result<(), TodoError> {
        let todo = self
            .todo
            .find_by_id(id)
            .await
            .map_err(TodoError::from)?
            .ok_or(TodoError::NotFound)?;

        self.todo
            .update(todo.id, dto.title, dto.description)
            .await
            .map_err(TodoError::from)
    }

    pub async fn delete_todo(&self, id: Uuid) -> Result<(), TodoError> {
        let todo = self
            .todo
            .find_by_id(id)
            .await
            .map_err(TodoError::from)?
            .ok_or(TodoError::NotFound)?;

        self.todo.delete(todo.id).await.map_err(TodoError::from)
    }

    pub async fn find_all(&self) -> Result<Vec<Todo>, TodoError> {
        self.todo.find_all().await.map_err(TodoError::from)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Todo>, TodoError> {
        self.todo.find_by_id(id).await.map_err(TodoError::from)
    }
}
