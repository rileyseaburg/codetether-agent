//! State reset and legacy tensor-input compatibility for the direct Bonsai decoder.
use super::Model;
use anyhow::{Result, ensure};
use candle_core::Tensor;
impl Model {
    pub fn forward(&mut self, input: &Tensor, pos: usize) -> Result<Tensor> {
        let (batch, count) = input.dims2()?;
        ensure!(
            batch == 1 && count > 0,
            "Bonsai requires one nonempty sequence"
        );
        self.forward_tokens(&input.to_vec2::<u32>()?[0], pos, &|| false)
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
