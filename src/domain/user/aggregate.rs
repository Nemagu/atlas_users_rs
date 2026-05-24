use uuid::Uuid;

use crate::domain::{
    aggregate::AggregateMeta,
    error::DomainError,
    user::value_object::{Email, UserId, UserState, UserStatus},
    value_object::Version,
};

#[derive(Debug, Clone)]
pub(crate) struct User {
    meta: AggregateMeta<UserId>,
    email: Email,
    status: UserStatus,
    state: UserState,
}

impl User {
    pub(crate) fn new(id: UserId, email: Email) -> Result<Self, DomainError> {
        Ok(Self {
            meta: AggregateMeta::new(id, Version::try_from(1)?),
            email,
            status: UserStatus::User,
            state: UserState::Active,
        })
    }

    pub(crate) fn restore(
        id: Uuid,
        email: String,
        status: &str,
        state: &str,
        version: u64,
    ) -> Result<Self, DomainError> {
        Ok(Self {
            meta: AggregateMeta::new(id.into(), version.try_into()?),
            email: email.try_into()?,
            status: status.try_into()?,
            state: state.try_into()?,
        })
    }

    pub(crate) fn id(&self) -> &UserId {
        &self.meta.id
    }

    pub(crate) fn email(&self) -> &Email {
        &self.email
    }

    pub(crate) fn status(&self) -> &UserStatus {
        &self.status
    }

    pub(crate) fn state(&self) -> &UserState {
        &self.state
    }

    pub(crate) fn version(&self) -> &Version {
        &self.meta.version
    }

    pub(crate) fn new_email(&mut self, email: Email) -> Result<(), DomainError> {
        self.check_state()?;
        if self.email == email {
            Err(DomainError::InvalidData(
                "current email is equal new email".into(),
            ))
        } else {
            self.email = email;
            self.meta.update_version();
            Ok(())
        }
    }

    pub(crate) fn new_status(&mut self, status: UserStatus) -> Result<(), DomainError> {
        self.check_state()?;
        if self.status == status {
            Err(DomainError::InvalidData(
                "current status is equal new status".into(),
            ))
        } else {
            self.status = status;
            self.meta.update_version();
            Ok(())
        }
    }

    pub(crate) fn appoin_admin(&mut self) -> Result<(), DomainError> {
        self.new_status(UserStatus::Admin)
    }

    pub(crate) fn appoint_user(&mut self) -> Result<(), DomainError> {
        self.new_status(UserStatus::User)
    }

    pub(crate) fn new_state(&mut self, state: UserState) -> Result<(), DomainError> {
        if self.state == state {
            Err(DomainError::InvalidData(
                "current state is equal new state".into(),
            ))
        } else {
            self.state = state;
            self.meta.update_version();
            Ok(())
        }
    }

    pub(crate) fn activate(&mut self) -> Result<(), DomainError> {
        self.new_state(UserState::Active)
    }

    pub(crate) fn freeze(&mut self) -> Result<(), DomainError> {
        self.new_state(UserState::Frozen)
    }

    pub(crate) fn delete(&mut self) -> Result<(), DomainError> {
        self.new_state(UserState::Deleted)
    }

    fn check_state(&self) -> Result<(), DomainError> {
        match self.state {
            UserState::Active => Ok(()),
            UserState::Frozen => Err(DomainError::InvalidData(
                "you can not change user, because he was frozen".into(),
            )),
            UserState::Deleted => Err(DomainError::InvalidData(
                "you can not change user, because he was deleted".into(),
            )),
        }
    }
}
