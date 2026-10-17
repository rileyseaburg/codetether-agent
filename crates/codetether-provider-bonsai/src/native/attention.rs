//! Single-token causal grouped-query attention with an explicit KV cache.
use super::linear::Linear;
use candle_core::Tensor;
pub(super) struct Attention {
    pub q: Linear,
    pub k: Linear,
    pub v: Linear,
    pub out: Linear,
    pub qnorm: Tensor,
    pub knorm: Tensor,
    pub cache: Option<(Tensor, Tensor)>,
}
