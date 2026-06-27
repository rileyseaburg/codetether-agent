//! # Vector DB
//!
//! A dependency-free, file-based vector store with typed payloads.
//!
//! Components:
//! - [`EmbeddingVector`] — L2-normalized vector newtype.
//! - [`LocalEmbeddingEngine`] — hashing-trick embeddings, no network.
//! - [`TextEmbedder`] — backend seam; swap the local engine for a real model.
//! - [`ProviderEmbedder`] — [`TextEmbedder`] backed by a learned model via a
//!   [`Provider`](crate::provider::Provider) (e.g. `text-embedding-3-small`).
//! - [`auto`] — automatic backend selection: probes host resources and uses a
//!   local HuggingFace model when capable, else a cloud embedding provider.
//! - [`VectorStore`] — generic-payload store with cosine [`search`](VectorStore::search)
//!   and JSON persistence.
//! - [`Embeddable`] — implement on a type to derive its embedding text.
//!
//! ## Quick Start
//!
//! ```rust
//! use codetether_agent::vectordb::{LocalEmbeddingEngine, VectorStore};
//!
//! let engine = LocalEmbeddingEngine::new(64);
//! let mut store: VectorStore<String> = VectorStore::new();
//! store.upsert("a", engine.embed("rust async runtime"), "a".into());
//! store.upsert("b", engine.embed("kubernetes deployment"), "b".into());
//!
//! let q = engine.embed("async tokio runtime");
//! let hits = store.search(&q, 1);
//! assert_eq!(hits[0].record.id, "a");
//! ```

pub mod auto;
pub mod embed;
pub mod embed_hash;
pub mod embeddable;
pub mod embedder;
pub mod persist;
pub mod provider;
pub mod record;
pub mod similarity;
pub mod store;
pub mod store_mutate;
pub mod store_search;
pub mod tokenize;
pub mod vector;

#[cfg(test)]
#[path = "provider_tests.rs"]
mod provider_tests;
#[cfg(test)]
mod tests;

pub use embed::{DEFAULT_DIMENSIONS, LocalEmbeddingEngine};
pub use embeddable::Embeddable;
pub use embedder::TextEmbedder;
pub use provider::ProviderEmbedder;
pub use record::Record;
pub use similarity::cosine;
pub use store::VectorStore;
pub use store_search::Hit;
pub use vector::{EmbeddingVector, l2_normalize};
