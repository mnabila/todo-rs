use anyhow::Result;
use chrono::Utc;
use uuid::Uuid;

use crate::{
    application::todo::dto::{CreateTodoDto, UpdateTodoDto},
    domain::todo::{error::TodoError, model::Todo, repository::TodoRepository},
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

        self.todo.create(&todo).await
    }

    pub async fn update_todo(&self, id: Uuid, dto: UpdateTodoDto) -> Result<(), TodoError> {
        let todo = Todo {
            id,
            title: dto.title,
            description: dto.description,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.todo.update(&todo).await
    }

    pub async fn delete_todo(&self, id: Uuid) -> Result<(), TodoError> {
        self.todo.delete(id).await
    }

    pub async fn find_all(&self) -> Result<Vec<Todo>, TodoError> {
        self.todo.find_all().await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Todo>, TodoError> {
        self.todo.find_by_id(id).await
    }
}
