//! Typed vector storage without an agent runtime or network dependency.
//!
//! [`VectorStore`] persists typed [`Record`] payloads as JSON and ranks them
//! with cosine similarity. [`LocalEmbeddingEngine`] creates deterministic
//! hashing-based vectors; implement [`TextEmbedder`] for other backends.
//! Provider selection and network-backed adapters belong to the calling app.
//!
//! # Examples
//!
//! ```rust
//! use codetether_vectordb::{LocalEmbeddingEngine, VectorStore};
//!
//! let engine = LocalEmbeddingEngine::new(64);
//! let mut store: VectorStore<String> = VectorStore::new();
//! store.upsert("a", engine.embed("rust async runtime"), "Rust".into());
//! let hits = store.search(&engine.embed("rust async runtime"), 1);
//! assert_eq!(hits[0].record.payload, "Rust");
//! ```

pub mod embed;
pub mod embed_hash;
pub mod embeddable;
pub mod embedder;
pub mod persist;
pub mod record;
pub mod similarity;
pub mod store;
pub mod store_mutate;
pub mod store_search;
pub mod tokenize;
pub mod vector;

pub use embed::{DEFAULT_DIMENSIONS, LocalEmbeddingEngine};
pub use embeddable::Embeddable;
pub use embedder::TextEmbedder;
pub use record::Record;
pub use similarity::cosine;
pub use store::VectorStore;
pub use store_search::Hit;
pub use vector::{EmbeddingVector, l2_normalize};

#[cfg(test)]
mod tests;