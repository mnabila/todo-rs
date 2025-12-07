use thiserror::Error;

#[derive(Error, Debug)]
pub enum TodoError {
    #[error("Todo not found")]
    NotFound,

    #[error("Cannot create todo : {0}")]
    CreateError(String),

    #[error("Cannot update todo : {0}")]
    UpdateError(String),

    #[error("Cannot delete todo : {0}")]
    DeleteError(String),
}
