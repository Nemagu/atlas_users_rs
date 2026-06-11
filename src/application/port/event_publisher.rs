use crate::application::{error::AppResult, port::dto::user::UserDTO};

#[async_trait::async_trait]
pub(crate) trait EventPublisher: Send + Sync {
    async fn publish_users(&self, users: &[UserDTO]) -> AppResult<()>;
}
