use crate::{
    application::error::{AppError, AppResult},
    domain::user::{aggregate::User, value_object::UserId},
};

#[derive(Debug, Clone)]
pub(crate) enum UserEvent {
    Create,
    Update,
    Freeze,
    Delete,
    Recover,
}

impl UserEvent {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            UserEvent::Create => "create",
            UserEvent::Update => "update",
            UserEvent::Freeze => "freeze",
            UserEvent::Delete => "delete",
            UserEvent::Recover => "recover",
        }
    }
}

impl TryFrom<&str> for UserEvent {
    type Error = AppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "create" => Self::Create,
            "update" => Self::Update,
            "freeze" => Self::Freeze,
            "delete" => Self::Delete,
            "recover" => Self::Recover,
            _ => {
                return Err(AppError::InvalidData(format!(
                    "invalid user event string - {value}"
                )));
            }
        })
    }
}

impl From<&UserEvent> for String {
    fn from(value: &UserEvent) -> Self {
        value.as_str().into()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Password(String);

impl Password {
    pub(crate) fn new(password: String) -> Self {
        Self(password)
    }

    pub(crate) fn into_inner(self) -> String {
        self.0
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Password {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PasswordHash(String);

impl PasswordHash {
    pub(crate) fn new(password_hash: String) -> Self {
        Self(password_hash)
    }

    pub(crate) fn into_inner(self) -> String {
        self.0
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for PasswordHash {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct UserDTO {
    pub(crate) user: User,
    pub(crate) editor_id: Option<UserId>,
    pub(crate) state: UserEvent,
    pub(crate) password_hash: Option<PasswordHash>,
}

impl UserDTO {
    pub(crate) fn new(
        user: User,
        editor_id: Option<UserId>,
        state: UserEvent,
        password_hash: Option<PasswordHash>,
    ) -> Self {
        Self {
            user,
            editor_id,
            state,
            password_hash,
        }
    }
}
