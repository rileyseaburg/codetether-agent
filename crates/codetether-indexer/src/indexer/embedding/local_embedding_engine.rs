//! local embedding engine for codebase indexing.

use super::*;

#[derive(Debug, Clone)]
pub(in crate::indexer) struct LocalEmbeddingEngine {
    pub(in crate::indexer) dimensions: usize,
}

impl LocalEmbeddingEngine {
    pub(in crate::indexer) fn new(dimensions: usize) -> Self {
        Self { dimensions }
    }

    pub(in crate::indexer) fn embed_batch(&self, inputs: &[String]) -> Vec<Vec<f32>> {
        inputs
            .iter()
            .map(|input| self.embed_single(input))
            .collect()
    }

    pub(in crate::indexer) fn accumulate_char_ngrams(&self, vector: &mut [f32], input: &str) {
        for ngram in input.as_bytes().windows(3).take(2048) {
            let key = String::from_utf8_lossy(ngram);
            self.accumulate_token(vector, &key, 0.5);
        }
    }

    pub(in crate::indexer) fn accumulate_token(
        &self,
        vector: &mut [f32],
        token: &str,
        weight: f32,
    ) {
        if token.is_empty() {
            return;
        }

        let digest = Sha256::digest(token.as_bytes());
        let len = vector.len();

        let idx_a = (u16::from_le_bytes([digest[0], digest[1]]) as usize) % len;
        let idx_b = (u16::from_le_bytes([digest[2], digest[3]]) as usize) % len;
        let idx_c = (u16::from_le_bytes([digest[4], digest[5]]) as usize) % len;

        let sign_a = if digest[6] & 1 == 0 { 1.0 } else { -1.0 };
        let sign_b = if digest[7] & 1 == 0 { 1.0 } else { -1.0 };
        let sign_c = if digest[8] & 1 == 0 { 1.0 } else { -1.0 };

        vector[idx_a] += sign_a * weight;
        vector[idx_b] += sign_b * (weight * 0.7);
        vector[idx_c] += sign_c * (weight * 0.4);
    }
}
mod single;
