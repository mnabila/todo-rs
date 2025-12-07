use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, post, put},
};
use sqlx::PgPool;

use crate::{
    application::todo::usecase::TodoUseCase,
    infrastructure::database::sqlx::todo_repository::PostgresTodoRepository,
    presentation::http::controller::todo::{
        create_todo, delete_todo, find_all_todo, find_todo_by_id, update_todo,
    },
};

#[derive(Clone)]
pub struct TodoState {
    pub todo: Arc<TodoUseCase<PostgresTodoRepository>>,
}

pub fn setup(pool: PgPool) -> Router {
    let repo = PostgresTodoRepository::new(pool.clone());
    let usecase = TodoUseCase::new(repo);

    let state = TodoState {
        todo: Arc::new(usecase),
    };

    Router::new()
        .route("/", post(create_todo))
        .route("/", get(find_all_todo))
        .route("/{id}", put(update_todo))
        .route("/{id}", delete(delete_todo))
        .route("/{id}", get(find_todo_by_id))
        .with_state(state)
}
