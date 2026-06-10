use crate::domain::error::DomainError;

#[derive(Debug, thiserror::Error)]
pub(crate) enum AppError {
    #[error("invalid data: {0}")]
    InvalidData(String),
    #[error("policy: {0}")]
    Policy(String),
    #[error("internal error: {0}")]
    InternalError(String),
}

impl From<DomainError> for AppError {
    fn from(value: DomainError) -> Self {
        match value {
            DomainError::InvalidData(d) => Self::InvalidData(d),
            DomainError::Policy(d) => Self::Policy(d),
            DomainError::Internal(d) => Self::InternalError(d),
        }
    }
}

pub(crate) type AppResult<T> = Result<T, AppError>;
