use crate::domain::{
    error::{DomainError, DomainResult},
    user::{
        aggregate::User,
        repository::UserReadRepository,
        value_object::{Email, UserId},
    },
};

#[derive(Debug, Clone, Copy)]
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

    pub(crate) async fn check_new_user(&mut self, user: &User) -> DomainResult<()> {
        self._check_id(user.id()).await?;
        self._check_email(user.email()).await
    }

    pub(crate) async fn check_email(&mut self, email: &Email) -> DomainResult<()> {
        self._check_email(email).await
    }

    async fn _check_id(&mut self, id: &UserId) -> DomainResult<()> {
        match self.repo.by_id(id).await? {
            None => Ok(()),
            Some(_) => Err(DomainError::InvalidData(
                "user with the same id exists".into(),
            )),
        }
    }

    async fn _check_email(&mut self, email: &Email) -> DomainResult<()> {
        match self.repo.by_email(email).await? {
            None => Ok(()),
            Some(_) => Err(DomainError::InvalidData(
                "user with the same email exists".into(),
            )),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct UserPolicyService;

impl UserPolicyService {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn edit_other(&self, initiator: &User) -> DomainResult<()> {
        self.check_state(initiator)?;
        self.check_role(initiator)
    }

    pub(crate) fn edit_self_private_fields(&self, initiator: &User) -> DomainResult<()> {
        self.check_state(initiator)?;
        self.check_role(initiator)
    }

    fn check_role(&self, initiator: &User) -> DomainResult<()> {
        if initiator.role().is_admin() {
            Ok(())
        } else {
            Err(DomainError::Policy("you can not edit other users".into()))
        }
    }

    fn check_state(&self, initiator: &User) -> DomainResult<()> {
        if initiator.state().is_active() {
            Ok(())
        } else {
            Err(DomainError::Policy("you can not edit other users".into()))
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use fake::{Fake, faker::internet::en::SafeEmail};
    use uuid::Uuid;

    use crate::domain::{
        user::value_object::{Birthday, UserRole, UserState},
        value_object::Version,
    };

    use super::*;

    struct InMemoryUserReadRepository {
        user_by_id: Option<User>,
        user_by_email: Option<User>,
    }

    impl InMemoryUserReadRepository {
        fn new(user_by_id: Option<User>, user_by_email: Option<User>) -> Self {
            Self {
                user_by_id,
                user_by_email,
            }
        }
    }

    #[async_trait::async_trait]
    impl UserReadRepository for InMemoryUserReadRepository {
        async fn by_id(&mut self, _: &UserId) -> DomainResult<Option<User>> {
            Ok(self.user_by_id.to_owned())
        }

        async fn by_email(&mut self, _: &Email) -> DomainResult<Option<User>> {
            Ok(self.user_by_email.to_owned())
        }
    }

    fn build_user(role: UserRole, state: UserState) -> User {
        User::new(
            UserId::from(Uuid::now_v7()),
            Email::try_from(SafeEmail().fake::<String>()).unwrap(),
            Birthday::try_from(NaiveDate::from_ymd_opt(1999, 7, 21).unwrap()).unwrap(),
            role,
            state,
            Version::try_from(1).unwrap(),
        )
    }

    #[tokio::test]
    async fn test_unique_new_user_ok() {
        let repo = InMemoryUserReadRepository::new(None, None);
        let mut service = UserUniquenessService::new(repo);
        let user = build_user(UserRole::User, UserState::Active);

        let result = service.check_new_user(&user).await;

        assert!(result.is_ok(), "valid user is invalid")
    }

    #[tokio::test]
    async fn test_unique_new_user_exists() {
        struct Case<'a> {
            repo: InMemoryUserReadRepository,
            msg: &'a str,
        }

        let cases = vec![
            Case {
                repo: InMemoryUserReadRepository::new(
                    Some(build_user(UserRole::User, UserState::Active)),
                    None,
                ),
                msg: "user with id exists",
            },
            Case {
                repo: InMemoryUserReadRepository::new(
                    None,
                    Some(build_user(UserRole::User, UserState::Active)),
                ),
                msg: "user with email exists",
            },
        ];

        for case in cases {
            let (repo, msg) = (case.repo, case.msg);
            let mut service = UserUniquenessService::new(repo);
            let user = build_user(UserRole::User, UserState::Active);

            let result = service.check_new_user(&user).await;

            assert!(result.is_err(), "{}", msg);
        }
    }

    #[tokio::test]
    async fn test_unique_email_does_not_exist() {
        let repo = InMemoryUserReadRepository::new(None, None);
        let mut service = UserUniquenessService::new(repo);
        let email = Email::try_from(SafeEmail().fake::<String>()).unwrap();

        let result = service.check_email(&email).await;

        assert!(result.is_ok(), "user with email exists");
    }

    #[tokio::test]
    async fn test_unique_email_exists() {
        let repo = InMemoryUserReadRepository::new(
            Some(build_user(UserRole::User, UserState::Active)),
            None,
        );
        let mut service = UserUniquenessService::new(repo);
        let email = Email::try_from(SafeEmail().fake::<String>()).unwrap();

        let result = service.check_email(&email).await;

        assert!(result.is_ok(), "user with email does not exist");
    }

    #[test]
    fn test_policy_user_can_edit_other_users() {
        let user = build_user(UserRole::Admin, UserState::Active);
        let service = UserPolicyService::new();

        let result = service.edit_other(&user);

        assert!(result.is_ok(), "active admin user can edit other users");
    }

    #[test]
    fn test_policy_user_can_not_edit_other_users() {
        struct Case<'a> {
            user: User,
            msg: &'a str,
        }
        let cases = vec![
            Case {
                user: build_user(UserRole::Admin, UserState::Frozen),
                msg: "frozen admin can not edit other users",
            },
            Case {
                user: build_user(UserRole::Admin, UserState::Deleted),
                msg: "deleted admin can not edit other users",
            },
            Case {
                user: build_user(UserRole::User, UserState::Active),
                msg: "active user can not edit other users",
            },
            Case {
                user: build_user(UserRole::User, UserState::Frozen),
                msg: "frozen user can not edit other users",
            },
            Case {
                user: build_user(UserRole::User, UserState::Deleted),
                msg: "deleted user can not edit other users",
            },
        ];
        let service = UserPolicyService::new();

        for case in cases {
            let (user, msg) = (case.user, case.msg);
            let result = service.edit_other(&user);

            assert!(result.is_err(), "{}", msg);
        }
    }

    #[test]
    fn test_policy_user_can_edit_self_private_fields() {
        let user = build_user(UserRole::Admin, UserState::Active);
        let service = UserPolicyService::new();

        let result = service.edit_self_private_fields(&user);

        assert!(
            result.is_ok(),
            "active admin user can edit self private fields"
        );
    }

    #[test]
    fn test_policy_user_can_not_edit_self_private_fields() {
        struct Case<'a> {
            user: User,
            msg: &'a str,
        }
        let cases = vec![
            Case {
                user: build_user(UserRole::Admin, UserState::Frozen),
                msg: "frozen admin can not edit self private fields",
            },
            Case {
                user: build_user(UserRole::Admin, UserState::Deleted),
                msg: "deleted admin can not edit self private fields",
            },
            Case {
                user: build_user(UserRole::User, UserState::Active),
                msg: "active user can not edit self private fields",
            },
            Case {
                user: build_user(UserRole::User, UserState::Frozen),
                msg: "frozen user can not edit self private fields",
            },
            Case {
                user: build_user(UserRole::User, UserState::Deleted),
                msg: "deleted user can not edit self private fields",
            },
        ];
        let service = UserPolicyService::new();

        for case in cases {
            let (user, msg) = (case.user, case.msg);
            let result = service.edit_other(&user);

            assert!(result.is_err(), "{}", msg);
        }
    }
}
