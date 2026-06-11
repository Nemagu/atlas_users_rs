use crate::application::{
    error::AppResult,
    port::{
        event_publisher::EventPublisher,
        repository::user::UserOutboxRepository,
        unit_of_work::{UnitOfWork, UnitOfWorkFactory},
    },
};

#[derive(Debug, Clone)]
pub(crate) struct PublishUserUseCase<UOWF, EP>
where
    UOWF: UnitOfWorkFactory,
    EP: EventPublisher,
{
    uow_factory: UOWF,
    event_publisher: EP,
}

impl<UOWF, EP> PublishUserUseCase<UOWF, EP>
where
    UOWF: UnitOfWorkFactory,
    EP: EventPublisher,
{
    pub(crate) fn new(uow_factory: UOWF, event_publisher: EP) -> Self {
        Self {
            uow_factory,
            event_publisher,
        }
    }

    pub(crate) async fn execute(&self) -> AppResult<()> {
        let mut uow = self.uow_factory.create().await?;
        uow.begin().await?;
        let (_, _, outbox_repo) = uow.user_repositories()?;
        let user_dtos = outbox_repo.not_published().await?;
        if user_dtos.is_empty() {
            uow.commit().await?;
            return Ok(());
        }
        if let Err(e) = self.event_publisher.publish_users(&user_dtos).await {
            uow.rollback().await?;
            return Err(e);
        }
        let users: Vec<_> = user_dtos.into_iter().map(|dto| dto.user).collect();
        outbox_repo.save_as_published(&users).await?;
        uow.commit().await?;
        Ok(())
    }
}
