use std::sync::Arc;

use crate::{
    infrastructure::{bootstrap, config},
    presentation::restapi::{self, RouterOption},
};

mod application;
mod domain;
mod infrastructure;
mod presentation;

#[tokio::main]
async fn main() {
    // load app configuration
    let conf = Arc::new(config::load().unwrap());

    // setup logger for this application
    bootstrap::logger(&conf).unwrap();

    // setup postgresql pool for this application
    let pool = Arc::new(bootstrap::sqlx(&conf).await.unwrap());

    // setup listener for this application
    let listener = bootstrap::listener(&conf).await.unwrap();

    // setup main router
    let app = restapi::setup_routers(&RouterOption {
        pool: pool,
        config: conf.clone(),
    });

    tracing::debug!("listen on {}", conf.server_addr());

    if let Err(err) = axum::serve(listener, app).await {
        tracing::error!("axum server error: {}", err);
        panic!("Server crashed");
    }
}
