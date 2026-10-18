//! Versioned prepared-index serialization shape.
use codetether_context_policy::summary_index::SummaryIndex;
use serde::{Deserialize, Serialize};

/// Persisted summary index tied to an append-compatible transcript prefix.
/// # Examples
/// ```
/// let prepared = codetether_prepared_context::Prepared {
///     schema_version: 1, message_count: 0, fingerprint: 0, generation: 0,
///     index: codetether_context_policy::summary_index::SummaryIndex::new(),
/// };
/// assert_eq!(prepared.schema_version, 1);
/// ```
#[derive(Serialize, Deserialize)]
pub struct Prepared {
    /// Serialization schema version accepted by the decoder.
    pub schema_version: u8,
    /// Length of the transcript prefix covered when this index was built.
    pub message_count: usize,
    /// Fingerprint of that prefix; later appended messages remain compatible.
    pub fingerprint: u64,
    /// Worker generation that produced this index.
    pub generation: u64,
    /// Prepared summaries keyed by transcript range.
    pub index: SummaryIndex,
}
