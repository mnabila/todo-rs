use crate::domain::shared::error::ModelError;

#[derive(Debug)]
pub enum TodoError {
    NotFound,
    BussinerError,
    GeneralError,
}

impl From<ModelError> for TodoError {
    fn from(err: ModelError) -> Self {
        match err {
            ModelError::NotFound => Self::NotFound,
            ModelError::Database(_) => Self::GeneralError,
            _ => Self::BussinerError,
        }
    }
}
