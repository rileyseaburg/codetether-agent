//! Dense attention execution, separate from stored weights/cache.
use super::{attention::Attention, config::Config, math, rope::Rope};
use anyhow::Result;
use candle_core::{DType, Tensor};
impl Attention {
    pub fn forward(&mut self, x: &Tensor, pos: usize, c: &Config, rope: &Rope) -> Result<Tensor> {
        let joint = self.q.forward(x)?.reshape((c.heads, 2, c.head))?;
        let q = joint.narrow(1, 0, 1)?.squeeze(1)?;
        let gate = joint
            .narrow(1, 1, 1)?
            .squeeze(1)?
            .reshape((1, c.heads * c.head))?;
        let q = rope
            .apply(&math::rms(&q, &self.qnorm, c.eps)?, pos)?
            .unsqueeze(1)?;
        let k = self.k.forward(x)?.reshape((c.kv_heads, c.head))?;
        let k = rope
            .apply(&math::rms(&k, &self.knorm, c.eps)?, pos)?
            .unsqueeze(1)?
            .to_dtype(DType::F16)?;
        let v = self
            .v
            .forward(x)?
            .reshape((c.kv_heads, 1, c.head))?
            .to_dtype(DType::F16)?;
        let (k, v) = match &self.cache {
            Some((oldk, oldv)) => (Tensor::cat(&[oldk, &k], 1)?, Tensor::cat(&[oldv, &v], 1)?),
            None => (k, v),
        };
        self.cache = Some((k.clone(), v.clone()));
        let count = k.dim(1)?;
        let repeats = c.heads / c.kv_heads;
        let k = k
            .to_dtype(DType::F32)?
            .unsqueeze(1)?
            .repeat((1, repeats, 1, 1))?
            .reshape((c.heads, count, c.head))?;
        let v = v
            .to_dtype(DType::F32)?
            .unsqueeze(1)?
            .repeat((1, repeats, 1, 1))?
            .reshape((c.heads, count, c.head))?;
        let scores = q
            .matmul(&k.transpose(1, 2)?.contiguous()?)?
            .affine(1.0 / (c.head as f64).sqrt(), 0.0)?;
        let probabilities = candle_nn::ops::softmax_last_dim(&scores)?;
        let output = probabilities.matmul(&v)?.reshape((1, c.heads * c.head))?;
        self.out.forward(&(output * math::sigmoid(&gate)?)?)
    }
}
