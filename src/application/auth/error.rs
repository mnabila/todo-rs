use crate::domain::shared::error::ModelError;

#[derive(Debug)]
pub enum AuthError {
    NotFound,
    Conflict,
    InvalidCredentials,
    TokenExpired,
    GeneralError,
}

impl From<ModelError> for AuthError {
    fn from(err: ModelError) -> Self {
        tracing::error!("auth error : {}", err);
        match err {
            ModelError::NotFound => Self::NotFound,
            ModelError::Conflict => Self::Conflict,
            _ => Self::GeneralError,
        }
    }
}
