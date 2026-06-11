use crate::{
    application::{error::AppResult, port::dto::user::UserDTO},
    domain::user::{
        aggregate::User, repository::UserReadRepository as DomainUserReadRepository,
        value_object::UserId,
    },
};

#[async_trait::async_trait]
pub(crate) trait UserReadRepository: DomainUserReadRepository + Send + Sync {
    async fn next_id(&mut self) -> AppResult<UserId>;
    async fn save(&mut self, user: &User) -> AppResult<()>;
}

#[async_trait::async_trait]
pub(crate) trait UserVersionRepository: Send + Sync {
    async fn save(&mut self, user: &UserDTO) -> AppResult<()>;
}

#[async_trait::async_trait]
pub(crate) trait UserOutboxRepository: Send + Sync {
    async fn save_as_not_published(&mut self, users: &[User]) -> AppResult<()>;
    async fn save_as_published(&mut self, users: &[User]) -> AppResult<()>;
    async fn not_published(&mut self) -> AppResult<Vec<UserDTO>>;
}
