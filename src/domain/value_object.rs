use crate::domain::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Version(u64);

impl TryFrom<u64> for Version {
    type Error = DomainError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(DomainError::InvalidData(
                "aggregate version need to be great then 0".into(),
            ))
        } else {
            Ok(Self(value))
        }
    }
}

impl From<Version> for u64 {
    fn from(value: Version) -> Self {
        value.0
    }
}

impl Version {
    pub(super) fn update(&mut self) {
        self.0 += 1
    }
}
