//! Native sequential prefill/decode and state reset.
use super::model::Model;
use anyhow::{Result, ensure};
use candle_core::Tensor;
impl Model {
    pub fn forward(&mut self, input: &Tensor, pos: usize) -> Result<Tensor> {
        let (batch, count) = input.dims2()?;
        ensure!(
            batch == 1 && count > 0 && pos == self.position,
            "Bonsai requires a contiguous single-sequence request"
        );
        ensure!(
            pos.checked_add(count)
                .is_some_and(|end| end <= self.config.context),
            "Bonsai runtime context exceeded"
        );
        let tokens = input.to_vec2::<u32>()?;
        let mut last = None;
        for token in &tokens[0] {
            let mut hidden = self.embedding.row(*token, &self.device)?;
            for layer in &mut self.layers {
                hidden = layer.forward(&hidden, self.position, &self.config, &self.rope)?;
            }
            self.position += 1;
            last = Some(hidden);
        }
        Ok(self
            .output
            .forward(&super::math::rms(
                &last.unwrap(),
                &self.norm,
                self.config.eps,
            )?)?
            .broadcast_add(&self.mask)?)
    }
    pub fn clear(&mut self) {
        for layer in &mut self.layers {
            layer.clear();
        }
        self.position = 0;
    }
    pub fn context(&self) -> usize {
        self.config.context
    }
}
