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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_equal_0() {
        let version = Version::try_from(0);
        assert!(version.is_err(), "version cannot be equal 0");
    }

    #[test]
    fn version_more_then_0() {
        let version = Version::try_from(1);
        assert!(version.is_ok(), "version can be more then 0");
        let version = version.unwrap();
        assert_eq!(version.0, 1, "expected version 1");
    }

    #[test]
    fn update_version() {
        let mut version = Version::try_from(1).unwrap();
        version.update();
        assert_eq!(version.0, 2, "expected updated version 2");
    }
}
