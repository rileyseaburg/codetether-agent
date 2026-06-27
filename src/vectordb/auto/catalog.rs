//! Catalog of local HuggingFace embedding models (BERT-architecture).

/// A HuggingFace embedding model usable with the candle BERT backend.
#[derive(Debug, Clone, Copy)]
pub struct ModelSpec {
    /// HuggingFace repo id, e.g. `mixedbread-ai/mxbai-embed-large-v1`.
    pub repo: &'static str,
    /// Output embedding dimensionality.
    pub dimensions: usize,
    /// Approximate weights size in bytes (for resource gating).
    pub approx_bytes: u64,
}

/// Highest-quality recent model (top MTEB, BERT-large, 1024-d).
pub const BEST: ModelSpec = ModelSpec {
    repo: "mixedbread-ai/mxbai-embed-large-v1",
    dimensions: 1024,
    approx_bytes: 1_340_000_000,
};

/// Compact model for lower-resource hosts (BERT-base, 384-d).
pub const COMPACT: ModelSpec = ModelSpec {
    repo: "BAAI/bge-small-en-v1.5",
    dimensions: 384,
    approx_bytes: 133_000_000,
};

/// Choose the best model that fits within `total_memory_bytes`.
///
/// Leaves headroom by requiring ~3x the weight size in RAM.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::vectordb::auto::catalog::{best_fitting, BEST, COMPACT};
///
/// assert_eq!(best_fitting(64 * 1024 * 1024 * 1024).repo, BEST.repo);
/// assert_eq!(best_fitting(5 * 1024 * 1024 * 1024).repo, COMPACT.repo);
/// ```
pub fn best_fitting(total_memory_bytes: u64) -> ModelSpec {
    if total_memory_bytes >= BEST.approx_bytes.saturating_mul(3) {
        BEST
    } else {
        COMPACT
    }
}
