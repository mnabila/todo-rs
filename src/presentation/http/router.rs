use axum::Router;
use sqlx::PgPool;

pub mod todo;

pub fn setup(pool: PgPool) -> Router {
    Router::new().nest("/api/v1/todo", todo::setup(pool))
}
