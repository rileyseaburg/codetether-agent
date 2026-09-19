//! Text-only partial NeoX RoPE; all MRoPE axes share the token position.
use candle_core::{Device, Result, Tensor};
pub(super) struct Rope {
    cos: Tensor,
    sin: Tensor,
    width: usize,
}
impl Rope {
    pub fn new(config: &super::config::Config, device: &Device) -> Result<Self> {
        let half = config.rotary / 2;
        let mut cos = Vec::new();
        let mut sin = Vec::new();
        for pos in 0..config.context {
            for i in 0..half {
                let a = pos as f64 / config.base.powf((2 * i) as f64 / config.rotary as f64);
                cos.push(a.cos() as f32);
                sin.push(a.sin() as f32);
            }
        }
        Ok(Self {
            cos: Tensor::from_vec(cos, (config.context, half), device)?,
            sin: Tensor::from_vec(sin, (config.context, half), device)?,
            width: config.rotary,
        })
    }
    pub fn apply(&self, x: &Tensor, pos: usize) -> Result<Tensor> {
        let (_, head) = x.dims2()?;
        let half = self.width / 2;
        let a = x.narrow(1, 0, half)?;
        let b = x.narrow(1, half, half)?;
        let cos = self.cos.get(pos)?.unsqueeze(0)?;
        let sin = self.sin.get(pos)?.unsqueeze(0)?;
        let left = (a.broadcast_mul(&cos)? - b.broadcast_mul(&sin)?)?;
        let right = (a.broadcast_mul(&sin)? + b.broadcast_mul(&cos)?)?;
        Tensor::cat(
            &[&left, &right, &x.narrow(1, self.width, head - self.width)?],
            1,
        )
    }
}
