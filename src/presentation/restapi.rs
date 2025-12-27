use axum::Router;
use sqlx::PgPool;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{infrastructure::configuration::AppConfig, presentation::restapi::swagger::ApiDoc};

mod auth;
mod middleware;
mod response;
mod swagger;
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

    Router::new()
        .nest("/api/v1", router)
        .merge(SwaggerUi::new("/docs").url("/openapi.json", ApiDoc::openapi()))
}
