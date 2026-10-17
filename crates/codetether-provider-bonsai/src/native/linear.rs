//! Packed linear projection with the checkpoint's activation-side rotation.
use super::matmul::Pq2;
use candle_core::Tensor;
pub(super) struct Linear {
    pub(super) weights: Tensor,
    pub(super) packed: Option<Pq2>,
    pub(super) signs: Option<Tensor>,
    pub(super) grouped: bool,
}

impl Linear {
    pub(super) fn from_packed(weights: Tensor, packed: Pq2, signs: Tensor, grouped: bool) -> Self {
        Self {
            weights,
            packed: Some(packed),
            signs: Some(signs),
            grouped,
        }
    }
    pub(super) fn from_dense(weights: Tensor) -> Self {
        Self {
            weights,
            packed: None,
            signs: None,
            grouped: false,
        }
    }
}
