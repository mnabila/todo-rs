use chrono::Utc;
use uuid::Uuid;

use crate::{
    application::todo::{
        dto::{CreateTodoDto, TodoResponse, UpdateTodoDto},
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

    pub async fn create_todo(&self, user_id: Uuid, dto: CreateTodoDto) -> Result<(), TodoError> {
        let todo = Todo {
            id: Uuid::new_v4(),
            user_id,
            title: dto.title,
            description: dto.description,
            is_completed: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.todo.create(&todo).await.map_err(TodoError::from)
    }

    pub async fn update_todo(
        &self,
        user_id: Uuid,
        id: Uuid,
        dto: UpdateTodoDto,
    ) -> Result<(), TodoError> {
        let todo = self
            .todo
            .find_by_id(user_id, id)
            .await
            .map_err(TodoError::from)?
            .ok_or(TodoError::NotFound)?;

        self.todo
            .update(todo.id, dto.title, dto.description)
            .await
            .map_err(TodoError::from)
    }

    pub async fn toggle_todo(&self, user_id: Uuid, id: Uuid) -> Result<(), TodoError> {
        self.todo.toggle(user_id, id).await.map_err(TodoError::from)
    }

    pub async fn delete_todo(&self, user_id: Uuid, id: Uuid) -> Result<(), TodoError> {
        let todo = self
            .todo
            .find_by_id(user_id, id)
            .await
            .map_err(TodoError::from)?
            .ok_or(TodoError::NotFound)?;

        self.todo.delete(todo.id).await.map_err(TodoError::from)
    }

    pub async fn find_all(&self, user_id: Uuid) -> Result<Vec<TodoResponse>, TodoError> {
        self.todo
            .find_all(user_id)
            .await
            .map_err(TodoError::from)
            .map(|todos| {
                todos
                    .into_iter()
                    .map(TodoResponse::from)
                    .collect::<Vec<TodoResponse>>()
            })
    }

    pub async fn find_by_id(
        &self,
        user_id: Uuid,
        id: Uuid,
    ) -> Result<Option<TodoResponse>, TodoError> {
        self.todo
            .find_by_id(user_id, id)
            .await
            .map_err(TodoError::from)
            .map(|todo| todo.map(TodoResponse::from))
    }
}
