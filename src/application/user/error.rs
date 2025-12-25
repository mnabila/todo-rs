use crate::domain::shared::error::ModelError;

#[derive(Debug)]
pub enum UserError {
    NotFound,
    BussinerError,
    GeneralError,
}

impl From<ModelError> for UserError {
    fn from(err: ModelError) -> Self {
        match err {
            ModelError::NotFound => Self::NotFound,
            ModelError::Database(_) => Self::GeneralError,
            _ => Self::BussinerError,
        }
    }
}
