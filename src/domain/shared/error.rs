use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("Not Found")]
    NotFound,

    #[error("Database error: {0}")]
    Database(String),

    #[error("Data already exist")]
    Conflict,
}
