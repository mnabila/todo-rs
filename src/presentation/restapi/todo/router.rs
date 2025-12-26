use std::sync::Arc;

use axum::{
    Extension, Router, middleware,
    routing::{delete, get, patch, post, put},
};

use crate::{
    application::todo::usecase::TodoUseCase,
    infrastructure::database::sqlx::todo_repository::PostgresTodoRepository,
    presentation::restapi::{
        RouterOption,
        middleware::jwt_middleware,
        todo::controller::{
            create_todo, delete_todo, find_all_todo, find_todo_by_id, toggle_todo, update_todo,
        },
    },
};

#[derive(Clone)]
pub struct TodoState {
    pub todo: Arc<TodoUseCase<PostgresTodoRepository>>,
}

pub fn setup(opt: &RouterOption) -> Router {
    let repo = PostgresTodoRepository::new(opt.pool.clone());
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
        .route("/{id}/toggle", patch(toggle_todo))
        .layer(middleware::from_fn(jwt_middleware))
        .layer(Extension(opt.config.jwt_secret.clone()))
        .with_state(state)
}
