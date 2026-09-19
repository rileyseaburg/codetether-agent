//! A native Qwen3.5 block with residual attention and gated feed-forward updates.
use super::{
    attention::Attention, config::Config, linear::Linear, linear_attention::Recurrent, math,
    rope::Rope,
};
use anyhow::Result;
use candle_core::Tensor;
pub(super) enum Mixer {
    Attention(Attention),
    Recurrent(Recurrent),
}
pub(super) struct Layer {
    pub norm: Tensor,
    pub post: Tensor,
    pub mixer: Mixer,
    pub up: Linear,
    pub gate: Linear,
    pub down: Linear,
}
impl Layer {
    pub fn forward(
        &mut self,
        input: &Tensor,
        pos: usize,
        c: &Config,
        rope: &Rope,
    ) -> Result<Tensor> {
        let normalized = math::rms(input, &self.norm, c.eps)?;
        let mixed = match &mut self.mixer {
            Mixer::Attention(a) => a.forward(&normalized, pos, c, rope)?,
            Mixer::Recurrent(a) => a.forward(&normalized, c)?,
        };
        let residual = (input + mixed)?;
        let normalized = math::rms(&residual, &self.post, c.eps)?;
        let activated =
            (math::silu(&self.gate.forward(&normalized)?)? * self.up.forward(&normalized)?)?;
        Ok((residual + self.down.forward(&activated)?)?)
    }
    pub fn clear(&mut self) {
        match &mut self.mixer {
            Mixer::Attention(a) => a.cache = None,
            Mixer::Recurrent(a) => {
                a.history = None;
                a.state = None;
            }
        }
    }
}
