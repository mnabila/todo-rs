use axum::Router;
use sqlx::PgPool;

mod response;
mod todo;
mod user;

pub fn setup(pool: PgPool) -> Router {
    let router = Router::new()
        .nest("/todo", todo::router::setup(pool.clone()))
        .nest("/user", user::router::setup(pool.clone()));

    Router::new().nest("/api/v1", router)
}
