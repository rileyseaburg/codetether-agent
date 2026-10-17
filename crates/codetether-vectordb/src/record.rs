//! A single stored vector record with a generic payload.

use super::vector::EmbeddingVector;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// A vector plus its associated payload, keyed by a stable id.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(bound = "P: Serialize + DeserializeOwned")]
pub struct Record<P> {
    /// Stable unique identifier for upsert/dedup.
    pub id: String,
    /// The L2-normalized embedding.
    pub vector: EmbeddingVector,
    /// Arbitrary caller payload (typed, never `serde_json::Value` soup).
    pub payload: P,
}

impl<P> Record<P> {
    /// Construct a new record.
    pub fn new(id: impl Into<String>, vector: EmbeddingVector, payload: P) -> Self {
        Self {
            id: id.into(),
            vector,
            payload,
        }
    }
}
