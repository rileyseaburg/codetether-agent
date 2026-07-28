//! IndexedDB deterministic listing operations.

use super::record::IndexedDbRecord;
use super::store::{DatabaseMap, IndexedDbStore};

impl IndexedDbStore {
    /// List all records for one origin in deterministic order.
    pub fn list_origin(&self, origin: &str) -> Vec<IndexedDbRecord> {
        let mut records = Vec::new();
        if let Some(databases) = self.origins.get(origin) {
            push_origin(origin, databases, &mut records);
        }
        records.sort();
        records
    }

    /// List every stored record in deterministic order.
    pub fn list_all(&self) -> Vec<IndexedDbRecord> {
        let mut records = Vec::new();
        for (origin, databases) in &self.origins {
            push_origin(origin, databases, &mut records);
        }
        records.sort();
        records
    }
}

fn push_origin(origin: &str, databases: &DatabaseMap, records: &mut Vec<IndexedDbRecord>) {
    for (database, stores) in databases {
        for (object_store, values) in stores {
            for (key, value) in values {
                records.push(IndexedDbRecord::new(
                    origin,
                    database,
                    object_store,
                    key,
                    value,
                ));
            }
        }
    }
}
