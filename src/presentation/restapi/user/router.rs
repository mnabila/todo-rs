use std::sync::Arc;

use axum::{
    Extension, Router, middleware,
    routing::{delete, get},
};

use crate::{
    application::user::usecase::UserUseCase,
    infrastructure::database::sqlx::user_repository::PostgresUserRepository,
    presentation::restapi::{
        RouterOption,
        middleware::jwt_middleware,
        user::controller::{delete_user, find_all_user, find_user_by_id},
    },
};

#[derive(Clone)]
pub struct UserState {
    pub user_usecase: Arc<UserUseCase<PostgresUserRepository>>,
}

pub fn setup(opt: &RouterOption, user_usecase: Arc<UserUseCase<PostgresUserRepository>>) -> Router {
    let state = UserState {
        user_usecase: user_usecase,
    };

    Router::new()
        .route("/", get(find_all_user))
        .route("/{id}", delete(delete_user))
        .route("/{id}", get(find_user_by_id))
        .layer(middleware::from_fn(jwt_middleware))
        .layer(Extension(opt.config.jwt_secret.clone()))
        .with_state(state)
}
