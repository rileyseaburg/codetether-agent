//! apply vectors for index construction.

use super::*;

pub(super) fn apply_vectors(
    data: &mut IndexInputs,
    backend: &EmbeddingBackend,
    start: usize,
    vectors: Vec<Vec<f32>>,
) -> Result<()> {
    for (offset, vector) in vectors.into_iter().enumerate() {
        let dim = vector.len() as u32;
        if dim != data.stats.embedding_dimensions {
            anyhow::bail!(
                "embedding dimension mismatch: expected {}, got {} (provider: {}, model: {})",
                data.stats.embedding_dimensions,
                dim,
                backend.provider_name(),
                backend.model_name(),
            );
        }

        data.files[start + offset].embedding = vector;
        data.stats.embedded_files += 1;
    }
    Ok(())
}
