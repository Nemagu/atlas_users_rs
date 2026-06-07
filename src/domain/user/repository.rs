use crate::domain::{
    error::DomainResult,
    user::{
        aggregate::User,
        value_object::{Email, UserId},
    },
};

#[async_trait::async_trait]
pub(crate) trait UserReadRepository {
    async fn by_id(&mut self, id: &UserId) -> DomainResult<Option<User>>;
    async fn by_email(&mut self, email: &Email) -> DomainResult<Option<User>>;
}
