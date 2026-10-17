//! Hybrid recurrent attention: convolution, normalized Q/K, decay/delta and gated RMS.
use super::linear::Linear;
use candle_core::Tensor;
pub(super) struct Recurrent {
    pub qkv: Linear,
    pub gate: Linear,
    pub alpha: Linear,
    pub beta: Linear,
    pub out: Linear,
    pub conv: Tensor,
    pub dt: Tensor,
    pub a: Tensor,
    pub norm: Tensor,
    pub history: Option<Tensor>,
    pub state: Option<Tensor>,
}
