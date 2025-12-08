use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get},
};
use sqlx::PgPool;

use crate::{
    application::user::usecase::UserUseCase,
    infrastructure::database::sqlx::user_repository::PostgresUserRepository,
    presentation::http::user::controller::{delete_user, find_all_user, find_user_by_id},
};

#[derive(Clone)]
pub struct UserState {
    pub user: Arc<UserUseCase<PostgresUserRepository>>,
}

pub fn setup(pool: PgPool) -> Router {
    let repo = PostgresUserRepository::new(pool);
    let usecase = UserUseCase::new(repo);

    let state = UserState {
        user: Arc::new(usecase),
    };

    Router::new()
        .route("/", get(find_all_user))
        .route("/{id}", delete(delete_user))
        .route("/{id}", get(find_user_by_id))
        .with_state(state)
}
