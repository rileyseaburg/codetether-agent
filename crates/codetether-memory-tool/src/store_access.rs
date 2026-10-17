//! store access operations.
use super::{AutoEmbedder, MemoryEntry, Store};
impl<B: AutoEmbedder> Store<B> {
    /// Add a new memory, computing and caching its embedding on insert.
    /// Insert a record, creating its local embedding if needed.
    ///
    /// # Arguments
    /// `entry` is the record to store.
    /// # Returns
    /// The inserted record ID.
    /// # Examples
    /// ```rust
    /// let mut store = codetether_memory_tool::MemoryStore::default();
    /// let _id = store.add(codetether_memory_tool::MemoryEntry::new("text", vec![]));
    /// ```
    pub fn add(&mut self, mut entry: MemoryEntry) -> String {
        entry.ensure_embedding();
        let id = entry.id.clone();
        self.entries.insert(id.clone(), entry);
        id
    }

    /// Get a memory by ID
    /// Retrieve a record and update its access metadata.
    ///
    /// # Arguments
    /// `id` is a complete ID or unique prefix.
    /// # Returns
    /// The matching record, if one exists.
    /// # Examples
    /// ```rust
    /// assert!(codetether_memory_tool::MemoryStore::default().get("missing").is_none());
    /// ```
    pub fn get(&mut self, id: &str) -> Option<MemoryEntry> {
        let id = self.resolve_id(id)?;
        let entry = self.entries.get_mut(&id)?;
        entry.touch();
        Some(entry.clone())
    }

    fn resolve_id(&self, id: &str) -> Option<String> {
        if self.entries.contains_key(id) {
            return Some(id.to_string());
        }
        let mut matches = self.entries.keys().filter(|key| key.starts_with(id));
        let found = matches.next()?.clone();
        matches.next().is_none().then_some(found)
    }

    /// Delete a memory
    /// Remove a record matching an ID or unique prefix.
    ///
    /// # Arguments
    /// `id` identifies the record.
    /// # Returns
    /// Whether a record was removed.
    /// # Examples
    /// ```rust
    /// assert!(!codetether_memory_tool::MemoryStore::default().delete("missing"));
    /// ```
    pub fn delete(&mut self, id: &str) -> bool {
        self.resolve_id(id)
            .and_then(|id| self.entries.remove(&id))
            .is_some()
    }
}
