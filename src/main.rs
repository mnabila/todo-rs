use crate::{
    infrastructure::{bootstrap, configuration},
    presentation::restapi::{self, RouterOption},
};

mod application;
mod domain;
mod infrastructure;
mod presentation;

#[tokio::main]
async fn main() {
    // load app configuration
    let conf = configuration::load().unwrap();

    // setup logger for this application
    bootstrap::logger(&conf).unwrap();

    // setup postgresql pool for this application
    let pool = bootstrap::sqlx(&conf).await.unwrap();

    // setup listener for this application
    let listener = bootstrap::listener(&conf).await.unwrap();

    // setup main router
    let app = restapi::setup(&RouterOption {
        pool: &pool,
        config: &conf,
    });

    tracing::debug!("listen on {}", conf.server_addr());
    if let Err(err) = axum::serve(listener, app).await {
        tracing::error!("axum server error: {}", err);
        panic!("Server crashed");
    }
}
