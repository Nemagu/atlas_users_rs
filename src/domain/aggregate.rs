use crate::domain::value_object::Version;

#[derive(Debug, Clone)]
pub(crate) struct AggregateMeta<Id>
where
    Id: Clone,
{
    pub(super) id: Id,
    pub(super) version: Version,
    pub(super) original_version: Version,
}

impl<Id> AggregateMeta<Id>
where
    Id: Clone,
{
    pub(super) fn new(id: Id, version: Version) -> Self {
        Self {
            id,
            version: version.clone(),
            original_version: version,
        }
    }

    pub(super) fn update_version(&mut self) {
        if self.version == self.original_version {
            self.version.update()
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    #[test]
    fn update_version() {
        let version = Version::try_from(2).unwrap();
        let original_version = Version::try_from(1).unwrap();
        let mut meta = AggregateMeta::new(Uuid::now_v7(), original_version.clone());
        meta.update_version();
        meta.update_version();
        assert_eq!(meta.version, version);
        assert_eq!(meta.original_version, original_version);
    }
}
