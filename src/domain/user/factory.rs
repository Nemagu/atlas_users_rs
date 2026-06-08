use chrono::NaiveDate;
use uuid::Uuid;

use crate::domain::{
    error::DomainResult,
    user::{
        aggregate::User,
        value_object::{Birthday, Email, UserId, UserRole, UserState},
    },
    value_object::Version,
};

#[derive(Debug, Clone, Copy)]
pub(crate) struct UserFactory;

impl UserFactory {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn create(id: UserId, email: Email, birthday: Birthday) -> User {
        User::new(
            id,
            email,
            birthday,
            UserRole::User,
            UserState::Active,
            Version::default(),
        )
    }

    pub(crate) fn restore(
        id: Uuid,
        email: String,
        birthday: NaiveDate,
        role: &str,
        state: &str,
        version: u64,
    ) -> DomainResult<User> {
        Ok(User::new(
            UserId::from(id),
            email.try_into()?,
            birthday.try_into()?,
            role.try_into()?,
            state.try_into()?,
            Version::try_from(version)?,
        ))
    }
}
