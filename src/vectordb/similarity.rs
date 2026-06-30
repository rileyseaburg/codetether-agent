//! Similarity math for normalized embedding vectors.

use super::vector::EmbeddingVector;

/// Cosine similarity between two L2-normalized vectors.
///
/// Because both inputs are unit-length, this reduces to their dot product.
/// Returns `0.0` when the dimensions differ or either side is empty.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::vectordb::{EmbeddingVector, cosine};
///
/// let a = EmbeddingVector::new(vec![1.0, 0.0]);
/// let b = EmbeddingVector::new(vec![1.0, 0.0]);
/// assert!((cosine(&a, &b) - 1.0).abs() < 1e-6);
///
/// let c = EmbeddingVector::new(vec![0.0, 1.0]);
/// assert!(cosine(&a, &c).abs() < 1e-6);
/// ```
pub fn cosine(a: &EmbeddingVector, b: &EmbeddingVector) -> f32 {
    let (a, b) = (a.as_slice(), b.as_slice());
    if a.is_empty() || a.len() != b.len() {
        return 0.0;
    }
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}
