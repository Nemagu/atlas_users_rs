use chrono::NaiveDate;
use uuid::Uuid;

use crate::domain::user::aggregate::User;

pub(crate) struct UserDTO {
    pub(crate) id: Uuid,
    pub(crate) email: String,
    pub(crate) birthday: NaiveDate,
    pub(crate) role: String,
    pub(crate) state: String,
    pub(crate) version: u64,
}

impl From<&User> for UserDTO {
    fn from(value: &User) -> Self {
        Self::new(
            value.id().into(),
            value.email().into(),
            value.birthday().into(),
            value.role().into(),
            value.state().into(),
            value.version().into(),
        )
    }
}

impl UserDTO {
    pub(crate) fn new(
        id: Uuid,
        email: String,
        birthday: NaiveDate,
        role: String,
        state: String,
        version: u64,
    ) -> Self {
        Self {
            id,
            email,
            birthday,
            role,
            state,
            version,
        }
    }
}
