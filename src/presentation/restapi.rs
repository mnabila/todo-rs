use axum::Router;
use sqlx::PgPool;

use crate::infrastructure::configuration::AppConfig;

mod auth;
mod middleware;
mod response;
mod todo;
mod user;

pub struct RouterOption<'ro> {
    pub pool: &'ro PgPool,
    pub config: &'ro AppConfig,
}

pub fn setup(opt: &RouterOption) -> Router {
    let router = Router::new()
        .nest("/auth", auth::router::setup(opt))
        .nest("/todo", todo::router::setup(opt))
        .nest("/user", user::router::setup(opt));

    Router::new().nest("/api/v1", router)
}
