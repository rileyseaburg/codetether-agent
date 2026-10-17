//! Per-request sampler construction for Candle inference.

use candle_transformers::generation::LogitsProcessor;

use super::candle_runtime::CandleThinker;

impl CandleThinker {
    /// Build a per-request sampler, advancing the seed so repeated prompts do
    /// not replay identical sampling noise.
    pub(super) fn new_logits_processor(&mut self) -> LogitsProcessor {
        let seed = self.seed.wrapping_add(self.request_index);
        self.request_index = self.request_index.wrapping_add(1);
        LogitsProcessor::new(
            seed,
            Some(self.temperature as f64),
            self.top_p.map(|v| v as f64),
        )
    }
}
