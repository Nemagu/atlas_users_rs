use crate::domain::{
    error::{DomainError, DomainResult},
    user::{
        repository::UserReadRepository,
        value_object::{Email, UserId},
    },
};

#[derive(Debug)]
pub(crate) struct UserUniquenessService<R>
where
    R: UserReadRepository,
{
    repo: R,
}

impl<R> UserUniquenessService<R>
where
    R: UserReadRepository,
{
    pub(crate) fn new(repo: R) -> Self {
        Self { repo }
    }

    async fn check_id(&mut self, id: &UserId) -> DomainResult<()> {
        let existing_user = self.repo.by_id(id).await?;
        match existing_user {
            None => Ok(()),
            Some(_) => Err(DomainError::InvalidData(
                "user with the same id exists".into(),
            )),
        }
    }

    async fn check_email(&mut self, email: &Email) -> DomainResult<()> {
        let existing_user = self.repo.by_email(email).await?;
        match existing_user {
            None => Ok(()),
            Some(_) => Err(DomainError::InvalidData(
                "user with the same email exists".into(),
            )),
        }
    }
}
