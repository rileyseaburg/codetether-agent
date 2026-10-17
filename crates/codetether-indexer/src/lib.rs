//! Persistent codebase indexes, source relationships, and embedding batches.
//!
//! Use [`build_index`] for local embeddings. Hosts that manage remote providers
//! supply [`EmbeddingResolver`] to [`build_index_with_resolver`]. CLI formatting
//! and credentials remain with the host application.

mod contracts;
mod indexer;
mod local_resolver;
pub use contracts::{EmbeddingProvider, EmbeddingResolver};
pub use indexer::*;

#[cfg(test)]
mod remote_tests;
