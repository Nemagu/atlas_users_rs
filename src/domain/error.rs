#[derive(Debug, Clone, thiserror::Error)]
pub(crate) enum DomainError {
    #[error("invalid data: {0}")]
    InvalidData(String),
}
