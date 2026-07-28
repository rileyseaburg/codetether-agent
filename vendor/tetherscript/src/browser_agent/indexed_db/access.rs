//! IndexedDB record access operations.

use super::store::IndexedDbStore;

impl IndexedDbStore {
    /// Insert or replace one record.
    pub fn put(
        &mut self,
        origin: &str,
        database: &str,
        object_store: &str,
        key: &str,
        value: &str,
    ) {
        self.origins
            .entry(origin.into())
            .or_default()
            .entry(database.into())
            .or_default()
            .entry(object_store.into())
            .or_default()
            .insert(key.into(), value.into());
    }

    /// Get one record value.
    pub fn get(&self, origin: &str, database: &str, object_store: &str, key: &str) -> Option<&str> {
        self.origins
            .get(origin)?
            .get(database)?
            .get(object_store)?
            .get(key)
            .map(String::as_str)
    }
}
