//! IndexedDB store container.

use std::collections::HashMap;

pub(crate) type KeyMap = HashMap<String, String>;
pub(crate) type StoreMap = HashMap<String, KeyMap>;
pub(crate) type DatabaseMap = HashMap<String, StoreMap>;
pub(crate) type OriginMap = HashMap<String, DatabaseMap>;

/// In-memory, deterministic IndexedDB-like storage.
///
/// The store keeps string values only. It models the browser scoping shape
/// agents need first: origin -> database -> object store -> key.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::IndexedDbStore;
///
/// let mut store = IndexedDbStore::new();
/// store.put("https://example.test", "app", "users", "1", "Ada");
/// assert_eq!(store.get("https://example.test", "app", "users", "1"), Some("Ada"));
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IndexedDbStore {
    pub(crate) origins: OriginMap,
}

impl IndexedDbStore {
    /// Create an empty store.
    ///
    /// # Returns
    ///
    /// An empty [`IndexedDbStore`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Return true when no records are stored.
    ///
    /// # Returns
    ///
    /// `true` if the store has no origin buckets.
    pub fn is_empty(&self) -> bool {
        self.origins.is_empty()
    }

    /// Remove every stored IndexedDB-like record.
    pub(crate) fn clear(&mut self) {
        self.origins.clear();
    }
}
