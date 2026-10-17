//! Memory records and deterministic hybrid retrieval.
//!
//! [MemoryEntry] retains metadata and cached embeddings. [search] combines
//! lexical matches with semantic scores through [fusion]. [scope_widen]
//! can retry an empty scoped search without its scope constraint. Persistence
//! and embedding-provider discovery are supplied by callers.
//!
//! ```rust
//! let mut entry = codetether_memory_core::MemoryEntry::new("use bounded output", vec!["tools".into()]);
//! entry.ensure_embedding();
//! assert!(entry.cached_embedding().is_some());
//! ```
mod embeddable;
mod entry;
mod entry_construct;
mod entry_methods;
mod fuse;
pub mod fusion;
pub mod query_match;
pub mod scope_widen;
pub mod search;
mod search_rank;
pub mod semantic;
pub use entry::MemoryEntry;
