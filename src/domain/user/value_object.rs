use chrono::{Datelike, Local, NaiveDate};
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
pub(crate) struct Birthday(NaiveDate);

impl TryFrom<NaiveDate> for Birthday {
    type Error = DomainError;

    fn try_from(value: NaiveDate) -> Result<Self, Self::Error> {
        if value.year() < 1900 {
            return Err(DomainError::InvalidData(
                "birthday need to be after 1900".into(),
            ));
        }
        if (Local::now().date_naive() - value).num_days() <= 365 * 6 {
            return Err(DomainError::InvalidData("you is too yang".into()));
        }
        Ok(Self(value))
    }
}

impl From<Birthday> for NaiveDate {
    fn from(value: Birthday) -> Self {
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
pub(crate) enum UserRole {
    Admin,
    User,
}

impl UserRole {
    pub(crate) fn is_admin(&self) -> bool {
        *self == Self::Admin
    }
}

impl TryFrom<&str> for UserRole {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "admin" => Ok(Self::Admin),
            "user" => Ok(Self::User),
            _ => Err(DomainError::InvalidData(format!(
                "invalid user role string - {value}. you can select admin or user"
            ))),
        }
    }
}

impl From<UserRole> for String {
    fn from(value: UserRole) -> Self {
        match value {
            UserRole::Admin => "admin".into(),
            UserRole::User => "user".into(),
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
                "invalid user state string - {value}. you can select active, frozen or deleted"
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_birthday() {
        let date = NaiveDate::from_ymd_opt(1999, 7, 21).unwrap();

        let birthday = Birthday::try_from(date);

        assert!(birthday.is_ok(), "{date} is invalid");
    }

    #[test]
    fn test_invalid_birthday() {
        let cases = vec![
            NaiveDate::from_ymd_opt(1899, 1, 1).unwrap(),
            Local::now().date_naive(),
        ];

        for d in cases {
            let birthday = Birthday::try_from(d);

            assert!(birthday.is_err(), "{d} is valid");
        }
    }

    #[test]
    fn test_valid_role() {
        let admin_raw = "admin";
        let user_raw = "user";

        let admin = UserRole::try_from(admin_raw).unwrap();
        let user = UserRole::try_from(user_raw).unwrap();

        assert_eq!(admin, UserRole::Admin, "expected admin role");
        assert_eq!(user, UserRole::User, "expected user role");
    }

    #[test]
    fn test_invalid_role() {
        let cases = vec!["invalid_role", "Admin", "User"];
        for v in cases {
            let invalid_role = UserRole::try_from(v);

            assert!(invalid_role.is_err(), "{v} is valid");
        }
    }

    #[test]
    fn test_role_to_string() {
        let expected_admin = "admin".to_string();
        let expected_user = "user".to_string();

        let admin: String = UserRole::Admin.into();
        let user: String = UserRole::User.into();

        assert!(
            admin.eq(&expected_admin),
            "invalid admin role string presentation"
        );
        assert!(
            user.eq(&expected_user),
            "invalid user role string presentation"
        );
    }

    #[test]
    fn test_valid_state() {
        let active_raw = "active";
        let frozen_raw = "frozen";
        let deleted_raw = "deleted";

        let active = UserState::try_from(active_raw).unwrap();
        let frozen = UserState::try_from(frozen_raw).unwrap();
        let deleted = UserState::try_from(deleted_raw).unwrap();

        assert_eq!(active, UserState::Active, "expected active state");
        assert_eq!(frozen, UserState::Frozen, "expected frozen state");
        assert_eq!(deleted, UserState::Deleted, "expected deleted state");
    }

    #[test]
    fn test_invalid_state() {
        let cases = vec!["invalid_state", "Active", "Frozen", "Deleted"];
        for v in cases {
            let invalid_state = UserState::try_from(v);

            assert!(invalid_state.is_err(), "{v} is valid");
        }
    }

    #[test]
    fn test_state_to_string() {
        let expected_active = "active".to_string();
        let expected_frozen = "frozen".to_string();
        let expected_deleted = "deleted".to_string();

        let active: String = UserState::Active.into();
        let frozen: String = UserState::Frozen.into();
        let deleted: String = UserState::Deleted.into();

        assert!(
            active.eq(&expected_active),
            "invalid active state string presentation"
        );
        assert!(
            frozen.eq(&expected_frozen),
            "invalid frozen state string presentation"
        );
        assert!(
            deleted.eq(&expected_deleted),
            "invalid deleted state string presentation"
        );
    }
}
