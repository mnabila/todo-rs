use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("{0} not found")]
    NotFound(&'static str),

    #[error("Error creating {0}: {1}")]
    CreateError(&'static str, String),

    #[error("Error updating {0}: {1}")]
    UpdateError(&'static str, String),

    #[error("Error deleting {0}: {1}")]
    DeleteError(&'static str, String),
}
