//! Persistent memory storage and tool actions with caller-owned backend selection.
//!
//! [Store] persists shared [MemoryEntry] values; [Runtime] exposes save, search,
//! get, list, tags, delete and statistics. [AutoEmbedder] supplies optional
//! provider discovery while ordinary tool actions use deterministic retrieval.
//!
//! ```rust
//! let mut store = codetether_memory_tool::MemoryStore::default();
//! store.add(codetether_memory_tool::MemoryEntry::new("remember this", vec![]));
//! assert_eq!(store.stats().total_entries, 1);
//! ```
mod actions;
mod auto_init;
mod embedder_handle;
mod factory;
mod git_scope;
mod runtime;
mod schema;
mod scope;
mod stats;
mod store;
mod store_access;
mod store_debug;
mod store_embed;
mod store_io;
mod store_query;
mod tool;
pub use codetether_memory_core::{MemoryEntry, fusion, query_match, scope_widen, semantic};
pub use factory::{AutoEmbedder, NoAutoEmbedder};
pub use runtime::Runtime;
pub use stats::MemoryStats;
pub use store::Store;
pub mod search;
/// Standalone store using deterministic local embeddings.
pub type MemoryStore = Store<NoAutoEmbedder>;
/// Standalone tool without automatic provider discovery.
pub type MemoryTool = Runtime<NoAutoEmbedder>;
#[cfg(test)]
use codetether_tool_core::Tool;
#[cfg(test)]
use serde_json::json;
#[cfg(test)]
mod tests;

#[cfg(test)]
mod serialization_tests;
