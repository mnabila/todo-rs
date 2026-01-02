use std::time::Duration;

use sqlx::postgres::PgPoolOptions;
use tokio::io;
use tokio::net::TcpListener;
use tracing::subscriber::SetGlobalDefaultError;

use crate::infrastructure::config::AppConfig;

pub fn logger(conf: &AppConfig) -> Result<(), SetGlobalDefaultError> {
    let level = match conf.log_level.to_ascii_lowercase().as_str() {
        "info" => tracing::Level::INFO,
        "warn" => tracing::Level::WARN,
        "error" => tracing::Level::ERROR,
        _ => tracing::Level::DEBUG,
    };

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(level)
        .with_target(false)
        .with_line_number(true)
        .with_file(true)
        .json()
        .finish();

    // setup global logger using tracing subscriber
    tracing::subscriber::set_global_default(subscriber)
}

pub async fn sqlx(conf: &AppConfig) -> Result<sqlx::PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .min_connections(1)
        .max_connections((num_cpus::get() * 2).try_into().unwrap_or(2))
        .acquire_timeout(Duration::from_secs(1))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(1800))
        .connect(conf.db_uri().as_str())
        .await
        .inspect_err(|err| {
            tracing::error!("Failed to create database pool: {}", err);
        })
}

pub async fn listener(conf: &AppConfig) -> Result<TcpListener, io::Error> {
    TcpListener::bind(conf.server_addr()).await.map_err(|err| {
        tracing::error!("Failed to bind to {}: {}", conf.server_addr(), err);
        err
    })
}
