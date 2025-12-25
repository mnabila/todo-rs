use std::sync::Arc;

use axum::{
    middleware, routing::{delete, get, post}, Extension, Router
};

use crate::{
    application::auth::usecase::AuthUseCase,
    infrastructure::database::sqlx::user_repository::PostgresUserRepository,
    presentation::restapi::{
        RouterOption,
        auth::controller::{login, logout, refresh_access_token, register, whoami},
        middleware::jwt_middleware,
    },
};

#[derive(Clone)]
pub struct AuthState {
    pub auth: Arc<AuthUseCase<PostgresUserRepository>>,
}

pub fn setup(opt: &RouterOption) -> Router {
    let user = PostgresUserRepository::new(opt.pool.clone());
    let usecase = AuthUseCase::new(user, opt.config.jwt_secret.clone(), opt.config.jwt_duration);

    let state = AuthState {
        auth: Arc::new(usecase),
    };

    let public = Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/refresh", post(refresh_access_token));

    let private = Router::new()
        .route("/whoami", get(whoami))
        .route("/logout", delete(logout))
        .layer(middleware::from_fn(jwt_middleware))
        .layer(Extension(opt.config.jwt_secret.clone()));

    Router::new().merge(public).merge(private).with_state(state)
}
