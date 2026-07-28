//! CacheStorage container types.

use super::CacheRecord;

/// Ordered CacheStorage records for one browser context.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CacheStore {
    pub(crate) records: Vec<CacheRecord>,
}

impl CacheStore {
    pub(crate) fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub(crate) fn clear(&mut self) {
        self.records.clear();
    }
}
