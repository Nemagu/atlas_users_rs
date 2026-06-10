#[derive(Debug, Clone, thiserror::Error)]
pub(crate) enum DomainError {
    #[error("invalid data: {0}")]
    InvalidData(String),
    #[error("policy: {0}")]
    Policy(String),
    #[error("internal: {0}")]
    Internal(String),
}

pub(crate) type DomainResult<T> = Result<T, DomainError>;
