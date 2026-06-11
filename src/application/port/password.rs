use crate::application::{
    error::AppResult,
    port::dto::user::{Password, PasswordHash},
};

#[async_trait::async_trait]
pub(crate) trait PasswordManager {
    async fn hash(&self, password: Password) -> PasswordHash;
    async fn verify(&self, password: Password, password_hash: PasswordHash) -> AppResult<()>;
    fn validate(&self, password: Password) -> AppResult<()>;
}
