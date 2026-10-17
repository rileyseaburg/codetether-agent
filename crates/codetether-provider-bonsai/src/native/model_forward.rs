//! Native sequential prefill/decode and state reset.
use super::model::Model;
use anyhow::{Result, ensure};
use candle_core::Tensor;
impl Model {
    /// Direct host-token entry; no CPU -> GPU -> CPU token-ID round trip.
    pub(crate) fn forward_tokens(
        &mut self,
        tokens: &[u32],
        pos: usize,
        cancelled: &dyn Fn() -> bool,
    ) -> Result<Tensor> {
        let count = tokens.len();
        ensure!(
            count > 0 && pos == self.position,
            "Bonsai requires contiguous positions"
        );
        ensure!(
            pos.checked_add(count)
                .is_some_and(|end| end <= self.config.context),
            "Bonsai runtime context exceeded"
        );
        let mut last = None;
        for token in tokens {
            ensure!(!cancelled(), "Bonsai inference cancelled");
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
}
