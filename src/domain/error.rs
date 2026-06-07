#[derive(Debug, Clone, thiserror::Error)]
pub(crate) enum DomainError {
    #[error("invalid data: {0}")]
    InvalidData(String),
    #[error("internal: {0}")]
    Internal(String),
}

pub(crate) type DomainResult<T> = Result<T, DomainError>;
