//! IndexedDB deletion operations.

use super::store::IndexedDbStore;

impl IndexedDbStore {
    /// Delete one record and return whether it existed.
    pub fn delete(&mut self, origin: &str, database: &str, object_store: &str, key: &str) -> bool {
        let removed = self
            .origins
            .get_mut(origin)
            .and_then(|databases| databases.get_mut(database))
            .and_then(|stores| stores.get_mut(object_store))
            .and_then(|records| records.remove(key))
            .is_some();
        self.prune_empty(origin, database, object_store);
        removed
    }

    fn prune_empty(&mut self, origin: &str, database: &str, object_store: &str) {
        let Some(databases) = self.origins.get_mut(origin) else {
            return;
        };
        let Some(stores) = databases.get_mut(database) else {
            return;
        };
        if stores
            .get(object_store)
            .is_some_and(|records| records.is_empty())
        {
            stores.remove(object_store);
        }
        if stores.is_empty() {
            databases.remove(database);
        }
        if databases.is_empty() {
            self.origins.remove(origin);
        }
    }
}
