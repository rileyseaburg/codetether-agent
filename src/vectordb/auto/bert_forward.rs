//! Forward pass, mean pooling, and [`TextEmbedder`] impl for [`BertEmbedder`].

use super::bert_embedder::{BertEmbedder, encode};
use crate::vectordb::{EmbeddingVector, TextEmbedder};
use anyhow::Result;
use async_trait::async_trait;
use candle_core::Tensor;

/// Run the model on `text` and mean-pool token states into one vector.
fn embed_one(embedder: &BertEmbedder, text: &str) -> Result<Vec<f32>> {
    let _guard = embedder.lock.lock().unwrap();
    let (input_ids, token_type_ids) = encode(embedder, text)?;
    let output = embedder.model.forward(&input_ids, &token_type_ids, None)?;
    let pooled = mean_pool(&output)?;
    Ok(pooled.contiguous()?.to_vec1::<f32>()?)
}

/// Mean-pool a `[1, seq, hidden]` tensor over the sequence axis to `[hidden]`.
fn mean_pool(hidden: &Tensor) -> Result<Tensor> {
    let (_batch, seq, _hidden) = hidden.dims3()?;
    let summed = hidden.sum(1)?;
    Ok((summed / seq as f64)?.squeeze(0)?)
}

#[async_trait]
impl TextEmbedder for BertEmbedder {
    fn embed(&self, input: &str) -> EmbeddingVector {
        match embed_one(self, input) {
            Ok(values) => EmbeddingVector::new(values),
            Err(e) => {
                tracing::warn!(error = %e, "bert embed failed; returning empty vector");
                EmbeddingVector::new(Vec::new())
            }
        }
    }

    async fn embed_batch(&self, inputs: &[String]) -> Result<Vec<EmbeddingVector>> {
        inputs
            .iter()
            .map(|text| embed_one(self, text).map(EmbeddingVector::new))
            .collect()
    }
}
