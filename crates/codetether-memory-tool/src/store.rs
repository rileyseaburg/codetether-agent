//! Serializable memory collection and runtime-only embedding state.
use super::{MemoryEntry, embedder_handle};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Persistent records and optional runtime embedding backend.
///
/// `B` supplies automatic backend selection; its type marker is never serialized.
/// The JSON schema contains only the memory entries, as before extraction.
///
/// # Examples
/// ```rust
/// let store = codetether_memory_tool::MemoryStore::default();
/// assert_eq!(store.stats().total_entries, 0);
/// ```
#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(bound = "")]
pub struct Store<B> {
    pub(crate) entries: HashMap<String, MemoryEntry>,
    /// Optional learned-model embedding backend (runtime-only, never persisted).
    #[serde(skip)]
    pub(crate) embedder: embedder_handle::EmbedderHandle,
    #[serde(skip)]
    factory: std::marker::PhantomData<B>,
}
