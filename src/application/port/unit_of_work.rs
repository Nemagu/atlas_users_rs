use crate::application::{
    error::AppResult,
    port::repository::user::{UserOutboxRepository, UserReadRepository, UserVersionRepository},
};

#[async_trait::async_trait]
pub(crate) trait UnitOfWorkFactory: Send + Sync {
    type UOW: UnitOfWork;

    async fn create(&self) -> AppResult<Self::UOW>;
}

#[async_trait::async_trait]
pub(crate) trait UnitOfWork: Send + Sync {
    type UserReadRepo: UserReadRepository;
    type UserVersionRepo: UserVersionRepository;
    type UserOutboxRepo: UserOutboxRepository;

    async fn begin(&mut self) -> AppResult<()>;
    async fn commit(self) -> AppResult<()>;
    async fn rollback(self) -> AppResult<()>;

    fn user_repositories(
        &mut self,
    ) -> AppResult<(
        &mut Self::UserReadRepo,
        &mut Self::UserVersionRepo,
        &mut Self::UserOutboxRepo,
    )>;
}
