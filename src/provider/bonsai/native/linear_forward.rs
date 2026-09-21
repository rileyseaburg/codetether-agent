//! Apply activation transforms and packed/dense linear projection.
use super::{linear::Linear, rotation::Rotation};
use anyhow::Result;
use candle_core::Tensor;
impl Linear {
    pub fn forward(&self, input: &Tensor) -> Result<Tensor> {
        let mut input = input.clone();
        if self.grouped {
            input = super::gdn_layout::grouped(&input, 16, 3, 128)?;
        }
        if let Some(signs) = &self.signs {
            input = Rotation { inverse: false }.apply(&input, signs)?;
        }
        Ok(match &self.packed {
            Some(op) => op.apply(&input, &self.weights)?,
            None => input.matmul(&self.weights.t()?)?,
        })
    }
}
