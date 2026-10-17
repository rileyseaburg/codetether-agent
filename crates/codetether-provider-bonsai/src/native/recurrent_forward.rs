//! One-token recurrent attention execution.
use super::{config::Config, linear_attention::Recurrent, math};
use anyhow::Result;
use candle_core::{DType, Tensor};
impl Recurrent {
    pub fn forward(&mut self, x: &Tensor, c: &Config) -> Result<Tensor> {
        let channels = (2 * c.keys + c.values) * c.state;
        let qkv = self.qkv.forward(x)?;
        let history = match &self.history {
            Some(h) => h.clone(),
            None => Tensor::zeros((c.conv - 1, channels), DType::F32, x.device())?,
        };
        let window = Tensor::cat(&[&history, &qkv], 0)?;
        self.history = Some(window.narrow(0, 1, c.conv - 1)?.contiguous()?);
        let convolved = math::silu(&(window * self.conv.t()?)?.sum(0)?)?;
        let q = convolved
            .narrow(0, 0, c.keys * c.state)?
            .reshape((c.keys, c.state))?;
        let k = convolved
            .narrow(0, c.keys * c.state, c.keys * c.state)?
            .reshape((c.keys, c.state))?;
        let v = convolved
            .narrow(0, 2 * c.keys * c.state, c.values * c.state)?
            .reshape((c.values, c.state))?;
        let q = math::l2(&q, c.eps)?.repeat((c.values / c.keys, 1))?;
        let k = math::l2(&k, c.eps)?.repeat((c.values / c.keys, 1))?;
        let log_decay = math::softplus(&self.alpha.forward(x)?.broadcast_add(&self.dt)?)?
            .broadcast_mul(&self.a)?;
        let beta = math::sigmoid(&self.beta.forward(x)?)?;
        let state = match &self.state {
            Some(s) => s.clone(),
            None => Tensor::zeros((c.values, c.state, c.state), DType::F32, x.device())?,
        };
        let (output, state) = super::gdn::step(&state, &q, &k, &v, &log_decay, &beta)?;
        self.state = Some(state);
        let gate = self.gate.forward(x)?.reshape((c.values, c.state))?;
        let output = (math::rms(&output, &self.norm, c.eps)? * math::silu(&gate)?)?
            .reshape((1, c.values * c.state))?;
        self.out.forward(&output)
    }
}
