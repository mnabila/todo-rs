use crate::{infrastructure::configuration, presentation::http};
use sqlx::PgPool;
use tokio::net::TcpListener;

mod application;
mod domain;
mod infrastructure;
mod presentation;

#[tokio::main]
async fn main() {
    // load app configuration
    let conf = configuration::load().unwrap();

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .with_line_number(true)
        .with_file(true)
        .json()
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    let pool = PgPool::connect(&conf.db_uri())
        .await
        .expect("Failed to connect to PostgreSQL database");

    // let app = infrastructure::::setup(pool);

    let app = http::setup(pool);

    let listener = match TcpListener::bind(conf.server_addr()).await {
        Ok(listener) => {
            tracing::info!("Server listening on http://{}", conf.server_addr());
            listener
        }
        Err(err) => {
            tracing::error!("Failed to bind to {}: {}", conf.server_addr(), err);
            panic!("Cannot start server");
        }
    };

    if let Err(err) = axum::serve(listener, app).await {
        tracing::error!("Axum server error: {}", err);
        panic!("Server crashed");
    }
}
