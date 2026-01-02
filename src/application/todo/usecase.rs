use chrono::Utc;
use uuid::Uuid;

use crate::{
    application::todo::{
        dto::{CreateTodoRequest, TodoResponse, UpdateTodoRequest},
        error::TodoError,
    },
    domain::todo::{model::Todo, repository::TodoRepository},
};

pub struct TodoUseCase<T: TodoRepository + Send + Sync> {
    todo_repository: T,
}

impl<T: TodoRepository> TodoUseCase<T> {
    pub fn new(todo: T) -> Self {
        Self {
            todo_repository: todo,
        }
    }

    pub async fn create_todo(
        &self,
        user_id: Uuid,
        dto: CreateTodoRequest,
    ) -> Result<TodoResponse, TodoError> {
        let todo = Todo {
            id: Uuid::new_v4(),
            user_id,
            title: dto.title,
            description: dto.description,
            is_completed: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.todo_repository
            .create(todo)
            .await
            .map_err(TodoError::from)
            .map(TodoResponse::from)
    }

    pub async fn update_todo(
        &self,
        user_id: Uuid,
        id: Uuid,
        dto: UpdateTodoRequest,
    ) -> Result<TodoResponse, TodoError> {
        let mut todo = self
            .todo_repository
            .find_by_id(user_id, id)
            .await
            .map_err(TodoError::from)?
            .ok_or(TodoError::NotFound)?;

        todo.title = dto.title;
        todo.description = dto.description;
        todo.updated_at = Utc::now();

        self.todo_repository
            .update(todo)
            .await
            .map_err(TodoError::from)
            .map(TodoResponse::from)
    }

    pub async fn toggle_todo(&self, user_id: Uuid, id: Uuid) -> Result<(), TodoError> {
        self.todo_repository
            .toggle(user_id, id)
            .await
            .map_err(TodoError::from)
    }

    pub async fn delete_todo(&self, user_id: Uuid, id: Uuid) -> Result<(), TodoError> {
        let todo = self
            .todo_repository
            .find_by_id(user_id, id)
            .await
            .map_err(TodoError::from)?
            .ok_or(TodoError::NotFound)?;

        self.todo_repository
            .delete(todo.id)
            .await
            .map_err(TodoError::from)
    }

    pub async fn find_all(&self, user_id: Uuid) -> Result<Vec<TodoResponse>, TodoError> {
        self.todo_repository
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
        self.todo_repository
            .find_by_id(user_id, id)
            .await
            .map_err(TodoError::from)
            .map(|todo| todo.map(TodoResponse::from))
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;

    use crate::{
        application::todo::{
            dto::{CreateTodoRequest, UpdateTodoRequest},
            usecase::TodoUseCase,
        },
        domain::{
            shared::error::ModelError,
            todo::{model::Todo, repository::MockTodoRepository},
        },
    };

    #[tokio::test]
    async fn create_todo_success() {
        let mut repo = MockTodoRepository::new();

        repo.expect_create()
            .return_once(|t| Box::pin(async move { Ok(t) }));

        let usecase = TodoUseCase::new(repo);

        let dto = CreateTodoRequest {
            title: "test".to_string(),
            description: "hell world".to_string(),
        };

        let user_id = Uuid::new_v4();

        let result = usecase.create_todo(user_id, dto);

        assert!(result.await.is_ok())
    }

    #[tokio::test]
    async fn create_todo_failed() {
        let mut repo = MockTodoRepository::new();

        repo.expect_create().return_once(|t| {
            if t.user_id.is_nil() {
                return Box::pin(async {
                    Err(ModelError::Database("missing field user_id".to_string()))
                });
            }

            Box::pin(async move { Ok(t) })
        });

        let usecase = TodoUseCase::new(repo);

        let dto = CreateTodoRequest {
            title: "test".to_string(),
            description: "hello world".to_string(),
        };

        let user_id = Uuid::nil();

        let result = usecase.create_todo(user_id, dto);

        assert!(result.await.is_err())
    }

    #[tokio::test]
    async fn update_todo_success() {
        let mut repo = MockTodoRepository::new();
        let todo_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        repo.expect_find_by_id()
            .withf(move |uid, tid| uid == &user_id && tid == &todo_id)
            .return_once(|uid, tid| {
                let todo = Todo {
                    id: tid,
                    user_id: uid,
                    title: "test".to_string(),
                    description: "hello world".to_string(),
                    is_completed: false,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                };
                Box::pin(async move { Ok(Some(todo)) })
            });

        repo.expect_update()
            .withf(move |t| t.id == todo_id)
            .return_once(|t| Box::pin(async move { Ok(t) }));

        let usecase = TodoUseCase::new(repo);

        let dto = UpdateTodoRequest {
            title: "test".to_string(),
            description: "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.".to_string(),
        };

        let result = usecase.update_todo(user_id, todo_id, dto);

        assert!(result.await.is_ok())
    }

    #[tokio::test]
    async fn update_todo_failed() {
        let mut repo = MockTodoRepository::new();
        let todo_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        repo.expect_find_by_id()
            .return_once(|_, _| Box::pin(async move { Err(ModelError::NotFound) }));

        repo.expect_update()
            .withf(move |t| t.id == todo_id && t.user_id == user_id)
            .return_once(|t| Box::pin(async move { Ok(t) }));

        let usecase = TodoUseCase::new(repo);

        let dto = UpdateTodoRequest {
            title: "test".to_string(),
            description: "hello world".to_string(),
        };

        let result = usecase.update_todo(user_id, todo_id, dto);

        assert!(result.await.is_err())
    }

    #[tokio::test]
    async fn delete_todo_success() {
        let mut repo = MockTodoRepository::new();
        let todo_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        repo.expect_find_by_id()
            .withf(move |uid, tid| uid == &user_id && tid == &todo_id)
            .return_once(|uid, tid| {
                let todo = Todo {
                    id: tid,
                    user_id: uid,
                    title: "test".to_string(),
                    description: "hello world".to_string(),
                    is_completed: false,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                };
                Box::pin(async move { Ok(Some(todo)) })
            });

        repo.expect_delete()
            .withf(move |tid| tid == &todo_id)
            .return_once(|_| Box::pin(async { Ok(()) }));

        let usecase = TodoUseCase::new(repo);

        let result = usecase.delete_todo(user_id, todo_id);

        assert!(result.await.is_ok())
    }

    #[tokio::test]
    async fn delete_todo_failed() {
        let mut repo = MockTodoRepository::new();
        let todo_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        repo.expect_find_by_id()
            .return_once(|_, _| Box::pin(async move { Err(ModelError::NotFound) }));

        repo.expect_delete()
            .withf(move |tid| tid == &todo_id)
            .return_once(|_| Box::pin(async { Ok(()) }));

        let usecase = TodoUseCase::new(repo);

        let result = usecase.delete_todo(user_id, todo_id);

        assert!(result.await.is_err())
    }

    #[tokio::test]
    async fn find_all_success() {
        let mut repo = MockTodoRepository::new();
        let user_id = Uuid::new_v4();

        repo.expect_find_all()
            .withf(move |uid| uid == &user_id)
            .return_once(|uid| {
                let todos = vec![
                    Todo {
                        id: Uuid::new_v4(),
                        user_id: uid,
                        title: "Buy groceries".to_string(),
                        description: "Milk, eggs, bread, and fruits".to_string(),
                        is_completed: false,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                    },
                    Todo {
                        id: Uuid::new_v4(),
                        user_id: uid,
                        title: "Finish project".to_string(),
                        description: "Complete the Rust backend implementation".to_string(),
                        is_completed: false,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                    },
                    Todo {
                        id: Uuid::new_v4(),
                        user_id: uid,
                        title: "Call mom".to_string(),
                        description: "Wish her happy birthday".to_string(),
                        is_completed: true,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                    },
                ];
                Box::pin(async move { Ok(todos) })
            });

        let usecase = TodoUseCase::new(repo);

        let result = usecase.find_all(user_id);

        assert!(result.await.is_ok())
    }

    #[tokio::test]
    async fn find_all_failed() {
        let mut repo = MockTodoRepository::new();
        let user_id = Uuid::new_v4();

        repo.expect_find_all()
            .withf(move |uid| uid == &user_id)
            .return_once(|_| Box::pin(async { Err(ModelError::NotFound) }));

        let usecase = TodoUseCase::new(repo);

        let result = usecase.find_all(user_id);

        assert!(result.await.is_err())
    }

    #[tokio::test]
    async fn find_by_id_success() {
        let mut repo = MockTodoRepository::new();
        let todo_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        repo.expect_find_by_id()
            .withf(move |uid, tid| uid == &user_id && tid == &todo_id)
            .return_once(|uid, tid| {
                let todo = Todo {
                    id: tid,
                    user_id: uid,
                    title: "test".to_string(),
                    description: "hello world".to_string(),
                    is_completed: false,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                };
                Box::pin(async move { Ok(Some(todo)) })
            });

        let usecase = TodoUseCase::new(repo);

        let result = usecase.find_by_id(user_id, todo_id);

        assert!(result.await.is_ok())
    }

    #[tokio::test]
    async fn find_by_id_failed() {
        let mut repo = MockTodoRepository::new();
        let todo_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        repo.expect_find_by_id()
            .withf(move |uid, tid| uid == &user_id && tid == &todo_id)
            .return_once(|_, _| Box::pin(async { Err(ModelError::NotFound) }));

        let usecase = TodoUseCase::new(repo);

        let result = usecase.find_by_id(user_id, todo_id);

        assert!(result.await.is_err())
    }

    #[tokio::test]
    async fn toggle_todo_success() {
        let mut repo = MockTodoRepository::new();
        let todo_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        repo.expect_toggle()
            .withf(move |uid, tid| uid == &user_id && tid == &todo_id)
            .return_once(|_, _| Box::pin(async { Ok(()) }));

        let usecase = TodoUseCase::new(repo);

        let result = usecase.toggle_todo(user_id, todo_id);

        assert!(result.await.is_ok())
    }

    #[tokio::test]
    async fn toggle_todo_failed() {
        let mut repo = MockTodoRepository::new();
        let todo_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        repo.expect_toggle()
            .withf(move |uid, tid| uid == &user_id && tid == &todo_id)
            .return_once(|_, _| Box::pin(async { Err(ModelError::NotFound) }));

        let usecase = TodoUseCase::new(repo);

        let result = usecase.toggle_todo(user_id, todo_id);

        assert!(result.await.is_err())
    }
}
