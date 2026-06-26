//! L2-normalized embedding vector newtype.
//!
//! Wrapping the raw `Vec<f32>` enforces the unit-length invariant at
//! construction so callers can treat cosine similarity as a plain dot product.

use serde::{Deserialize, Serialize};

/// A unit-length (L2-normalized) embedding vector.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::vectordb::EmbeddingVector;
///
/// let v = EmbeddingVector::new(vec![3.0, 4.0]);
/// let norm: f32 = v.as_slice().iter().map(|x| x * x).sum::<f32>().sqrt();
/// assert!((norm - 1.0).abs() < 1e-6);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmbeddingVector(Vec<f32>);

impl EmbeddingVector {
    /// Build a normalized vector from raw values.
    pub fn new(mut values: Vec<f32>) -> Self {
        l2_normalize(&mut values);
        Self(values)
    }

    /// Number of dimensions.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Whether the vector is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Borrow the underlying values.
    pub fn as_slice(&self) -> &[f32] {
        &self.0
    }
}

/// Scale `values` to unit L2 length in place (no-op for the zero vector).
pub fn l2_normalize(values: &mut [f32]) {
    let norm = values.iter().map(|v| v * v).sum::<f32>().sqrt();
    if norm > 0.0 {
        for value in values {
            *value /= norm;
        }
    }
}
