//! Token sampling with a greedy recovery path for degenerate logits.

use anyhow::{Context, Result, anyhow};
use candle_core::Tensor;
use candle_transformers::generation::LogitsProcessor;

/// Sample the next token, recovering greedily when weights are not finite.
///
/// # Errors
///
/// Returns an error when sampling fails and no finite logit remains.
pub(super) fn sample_next_token(
    logits_processor: &mut LogitsProcessor,
    logits: &Tensor,
) -> Result<u32> {
    match logits_processor.sample(logits) {
        Ok(token) => Ok(token),
        Err(sample_error) => {
            let token = greedy_argmax(logits)
                .context("token sampling failed and greedy recovery found no finite logit")?;
            tracing::warn!(
                error = %sample_error,
                token,
                "Token sampling produced invalid weights; using greedy argmax"
            );
            Ok(token)
        }
    }
}

/// Pick the highest finite logit index, if any.
fn greedy_argmax(logits: &Tensor) -> Result<u32> {
    let values = logits
        .to_vec1::<f32>()
        .context("greedy recovery could not extract logits")?;
    values
        .into_iter()
        .enumerate()
        .filter(|(_, logit)| logit.is_finite())
        .max_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(idx, _)| idx as u32)
        .ok_or_else(|| anyhow!("all logits were non-finite"))
}
