use uuid::Uuid;

use crate::domain::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct UserId(Uuid);

impl From<Uuid> for UserId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<UserId> for Uuid {
    fn from(value: UserId) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Email(String);

impl TryFrom<String> for Email {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }
}

impl From<Email> for String {
    fn from(value: Email) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum UserStatus {
    Admin,
    User,
}

impl UserStatus {
    pub(crate) fn is_admin(&self) -> bool {
        *self == Self::Admin
    }
}

impl TryFrom<&str> for UserStatus {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "admin" => Ok(Self::Admin),
            "user" => Ok(Self::User),
            _ => Err(DomainError::InvalidData(format!(
                "invalid user status string - {value}. you can use admin or user"
            ))),
        }
    }
}

impl From<UserStatus> for String {
    fn from(value: UserStatus) -> Self {
        match value {
            UserStatus::Admin => "admin".into(),
            UserStatus::User => "user".into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum UserState {
    Active,
    Frozen,
    Deleted,
}

impl UserState {
    pub(crate) fn is_frozen(&self) -> bool {
        *self == Self::Frozen
    }

    pub(crate) fn is_deleted(&self) -> bool {
        *self == Self::Deleted
    }
}

impl TryFrom<&str> for UserState {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "active" => Ok(Self::Active),
            "frozen" => Ok(Self::Frozen),
            "deleted" => Ok(Self::Deleted),
            _ => Err(DomainError::InvalidData(format!(
                "invalid user state string - {value}. you can use active, frozen or deleted"
            ))),
        }
    }
}

impl From<UserState> for String {
    fn from(value: UserState) -> Self {
        match value {
            UserState::Active => "active".into(),
            UserState::Frozen => "frozen".into(),
            UserState::Deleted => "deleted".into(),
        }
    }
}
