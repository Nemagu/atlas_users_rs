use chrono::NaiveDate;
use uuid::Uuid;

use crate::domain::{
    aggregate::AggregateMeta,
    error::{DomainError, DomainResult},
    user::value_object::{Birthday, Email, UserId, UserRole, UserState},
    value_object::Version,
};

#[derive(Debug, Clone)]
pub(crate) struct User {
    meta: AggregateMeta<UserId>,
    email: Email,
    birthday: Birthday,
    role: UserRole,
    state: UserState,
}

impl User {
    pub(crate) fn new(
        id: UserId,
        email: Email,
        birthday: Birthday,
        role: UserRole,
        state: UserState,
        version: Version,
    ) -> Self {
        Self {
            meta: AggregateMeta::new(id, version),
            email,
            birthday,
            role,
            state,
        }
    }

    pub(crate) fn id(&self) -> &UserId {
        &self.meta.id
    }

    pub(crate) fn email(&self) -> &Email {
        &self.email
    }

    pub(crate) fn birthday(&self) -> &Birthday {
        &self.birthday
    }

    pub(crate) fn role(&self) -> &UserRole {
        &self.role
    }

    pub(crate) fn state(&self) -> &UserState {
        &self.state
    }

    pub(crate) fn version(&self) -> &Version {
        &self.meta.version
    }

    pub(crate) fn new_email(&mut self, email: Email) -> DomainResult<()> {
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

    pub(crate) fn new_birthday(&mut self, birthday: Birthday) -> DomainResult<()> {
        self.check_state()?;
        if self.birthday == birthday {
            Err(DomainError::InvalidData(
                "current birthday is equal new birthday".into(),
            ))
        } else {
            self.birthday = birthday;
            self.meta.update_version();
            Ok(())
        }
    }

    pub(crate) fn new_role(&mut self, role: UserRole) -> DomainResult<()> {
        self.check_state()?;
        if self.role == role {
            Err(DomainError::InvalidData(
                "current status is equal new role".into(),
            ))
        } else {
            self.role = role;
            self.meta.update_version();
            Ok(())
        }
    }

    pub(crate) fn appoint_admin(&mut self) -> DomainResult<()> {
        self.new_role(UserRole::Admin)
    }

    pub(crate) fn appoint_user(&mut self) -> DomainResult<()> {
        self.new_role(UserRole::User)
    }

    pub(crate) fn new_state(&mut self, state: UserState) -> DomainResult<()> {
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

    pub(crate) fn activate(&mut self) -> DomainResult<()> {
        self.new_state(UserState::Active)
    }

    pub(crate) fn freeze(&mut self) -> DomainResult<()> {
        self.new_state(UserState::Frozen)
    }

    pub(crate) fn delete(&mut self) -> DomainResult<()> {
        self.new_state(UserState::Deleted)
    }

    fn check_state(&self) -> DomainResult<()> {
        match self.state {
            UserState::Active => Ok(()),
            UserState::Frozen => Err(DomainError::InvalidData(
                "you can not change user, because it was frozen".into(),
            )),
            UserState::Deleted => Err(DomainError::InvalidData(
                "you can not change user, because it was deleted".into(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use fake::Fake;
    use fake::faker::internet::en::SafeEmail;

    use super::*;

    fn valid_email() -> Email {
        Email::try_from(SafeEmail().fake::<String>()).unwrap()
    }

    fn valid_birthday(date: Option<NaiveDate>) -> Birthday {
        match date {
            None => Birthday::try_from(NaiveDate::from_ymd_opt(1999, 7, 21).unwrap()).unwrap(),
            Some(d) => Birthday::try_from(d).unwrap(),
        }
    }

    fn build_user() -> User {
        User::new(
            UserId::from(Uuid::now_v7()),
            valid_email(),
            valid_birthday(None),
            UserRole::User,
            UserState::Active,
            Version::default(),
        )
    }

    #[test]
    fn test_new_email_ok() {
        let mut user = build_user();
        let new_email = valid_email();

        let result = user.new_email(new_email.clone());

        assert!(result.is_ok(), "error after updating user");
        assert_eq!(*user.email(), new_email, "user did not save new email");
    }

    #[test]
    fn test_new_email_is_equal() {
        let mut user = build_user();
        let new_email = user.email().to_owned();

        let result = user.new_email(new_email);

        assert!(
            result.is_err(),
            "user did not return error with the same email"
        );
    }

    #[test]
    fn test_new_email_when_state_is_not_active() {
        let cases = vec![UserState::Frozen, UserState::Deleted];

        for s in cases {
            let mut user = build_user();
            let _ = user.new_state(s.clone());
            let new_email = valid_email();

            let result = user.new_email(new_email);

            assert!(
                result.is_err(),
                "user did not return error with {} state",
                String::from(s)
            );
        }
    }

    #[test]
    fn test_new_birthday_ok() {
        let mut user = build_user();
        let new_birthday = valid_birthday(Some(NaiveDate::from_ymd_opt(2000, 7, 21).unwrap()));

        let result = user.new_birthday(new_birthday.clone());

        assert!(result.is_ok(), "error after updating user");
        assert_eq!(
            *user.birthday(),
            new_birthday,
            "user did not save new birthday"
        );
    }

    #[test]
    fn test_new_birthday_is_equal() {
        let mut user = build_user();
        let new_birthday = user.birthday().to_owned();

        let result = user.new_birthday(new_birthday);

        assert!(
            result.is_err(),
            "user did not return error with the same birthday"
        );
    }

    #[test]
    fn test_new_birthday_when_state_is_not_active() {
        let cases = vec![UserState::Frozen, UserState::Deleted];

        for s in cases {
            let mut user = build_user();
            let _ = user.new_state(s.clone());
            let new_birthday = valid_birthday(Some(NaiveDate::from_ymd_opt(2000, 7, 21).unwrap()));

            let result = user.new_birthday(new_birthday);

            assert!(
                result.is_err(),
                "user did not return error with {} state",
                String::from(s)
            );
        }
    }

    #[test]
    fn test_new_role_ok() {
        let mut user = build_user();
        let new_role = UserRole::Admin;

        let result = user.new_role(new_role.clone());

        assert!(result.is_ok(), "error after updating user");
        assert_eq!(*user.role(), new_role, "user did not save new role");
    }

    #[test]
    fn test_new_role_is_equal() {
        let mut user = build_user();
        let new_role = user.role().to_owned();

        let result = user.new_role(new_role);

        assert!(
            result.is_err(),
            "user did not return error with the same role"
        );
    }

    #[test]
    fn test_new_role_when_state_is_not_active() {
        let cases = vec![UserState::Frozen, UserState::Deleted];

        for s in cases {
            let mut user = build_user();
            let _ = user.new_state(s.clone());
            let new_role = UserRole::Admin;

            let result = user.new_role(new_role);

            assert!(
                result.is_err(),
                "user did not return error with {} state",
                String::from(s)
            );
        }
    }

    #[test]
    fn test_new_state_ok() {
        let mut user = build_user();
        let new_state = UserState::Frozen;

        let result = user.new_state(new_state.clone());

        assert!(result.is_ok(), "error after updating user");
        assert_eq!(*user.state(), new_state, "user did not save new state");
    }

    #[test]
    fn test_new_state_is_equal() {
        let mut user = build_user();
        let new_state = user.state().to_owned();

        let result = user.new_state(new_state);

        assert!(
            result.is_err(),
            "user did not return error with the same state"
        );
    }
}
