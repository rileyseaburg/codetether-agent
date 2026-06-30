//! Mutation operations for [`VectorStore`].

use super::persist;
use super::record::Record;
use super::store::VectorStore;
use super::vector::EmbeddingVector;
use anyhow::Result;
use serde::Serialize;
use serde::de::DeserializeOwned;

impl<P> VectorStore<P>
where
    P: Serialize + DeserializeOwned + Clone,
{
    /// Insert or replace the record with `id`.
    pub fn upsert(&mut self, id: impl Into<String>, vector: EmbeddingVector, payload: P) {
        let id = id.into();
        if let Some(slot) = self.records.iter_mut().find(|r| r.id == id) {
            slot.vector = vector;
            slot.payload = payload;
        } else {
            self.records.push(Record::new(id, vector, payload));
        }
    }

    /// Remove the record with `id`, returning whether one was removed.
    pub fn remove(&mut self, id: &str) -> bool {
        let before = self.records.len();
        self.records.retain(|r| r.id != id);
        before != self.records.len()
    }

    /// Persist to the backing file, if one is configured.
    pub fn save(&self) -> Result<()> {
        if let Some(path) = &self.path {
            persist::save(path, &self.records)?;
        }
        Ok(())
    }
}
