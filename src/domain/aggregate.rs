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
