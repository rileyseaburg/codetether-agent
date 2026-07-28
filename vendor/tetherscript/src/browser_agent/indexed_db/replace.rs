//! IndexedDB store replacement helpers.

use super::record::IndexedDbRecord;
use super::store::IndexedDbStore;

impl IndexedDbStore {
    pub(crate) fn replace_all(&mut self, records: Vec<IndexedDbRecord>) {
        self.origins.clear();
        for record in records {
            self.put(
                &record.origin,
                &record.database,
                &record.object_store,
                &record.key,
                &record.value,
            );
        }
    }
}
