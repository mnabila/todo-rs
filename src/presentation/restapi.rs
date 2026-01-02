use std::sync::Arc;

use axum::Router;
use sqlx::PgPool;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    application::{
        auth::usecase::AuthUseCase, todo::usecase::TodoUseCase, user::usecase::UserUseCase,
    },
    infrastructure::{
        config::AppConfig,
        database::sqlx::{
            todo_repository::PostgresTodoRepository, user_repository::PostgresUserRepository,
        },
    },
    presentation::restapi::swagger::ApiDoc,
};

mod auth;
mod middleware;
mod response;
mod swagger;
mod todo;
mod user;

pub struct RouterOption {
    pub pool: Arc<PgPool>,
    pub config: Arc<AppConfig>,
}

pub fn setup_routers(opt: &RouterOption) -> Router {
    let user_repo = Arc::new(PostgresUserRepository::new(opt.pool.clone()));
    let todo_repo = PostgresTodoRepository::new(opt.pool.clone());

    let auth_usecase = Arc::new(AuthUseCase::new(
        user_repo.as_ref().clone(),
        opt.config.jwt_secret.clone(),
        opt.config.jwt_duration,
    ));

    let todo_usecase = Arc::new(TodoUseCase::new(todo_repo));
    let user_usecase = Arc::new(UserUseCase::new(user_repo.as_ref().clone()));

    let router = Router::new()
        .nest("/auth", auth::router::setup(opt, auth_usecase))
        .nest("/todo", todo::router::setup(opt, todo_usecase))
        .nest("/user", user::router::setup(opt, user_usecase));

    Router::new()
        .nest("/api/v1", router)
        .merge(SwaggerUi::new("/docs").url("/openapi.json", ApiDoc::openapi()))
}
