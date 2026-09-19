//! Native text-only Bonsai decoder state; prefill is a correctness-first token loop.
use super::{config::Config, embedding::Embedding, layer::Layer, linear::Linear, rope::Rope};
use candle_core::{Device, Tensor};
pub(in crate::cognition::thinker::candle) struct Model {
    pub(super) config: Config,
    pub(super) embedding: Embedding,
    pub(super) output: Linear,
    pub(super) norm: Tensor,
    pub(super) mask: Tensor,
    pub(super) layers: Vec<Layer>,
    pub(super) rope: Rope,
    pub(super) device: Device,
    pub(super) position: usize,
}
