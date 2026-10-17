//! Focused embedding operation.

use super::*;

impl LocalEmbeddingEngine {
    pub(in crate::indexer) fn embed_single(&self, input: &str) -> Vec<f32> {
        let mut vector = vec![0.0f32; self.dimensions];
        let tokens = tokenize_for_embedding(input);

        if tokens.is_empty() {
            self.accumulate_char_ngrams(&mut vector, input);
        } else {
            for (idx, token) in tokens.iter().enumerate() {
                let positional_weight = 1.0f32 / (1.0 + (idx as f32 / 128.0));
                self.accumulate_token(&mut vector, token, positional_weight);

                if let Some(next) = tokens.get(idx + 1) {
                    let bigram = format!("{token} {next}");
                    self.accumulate_token(&mut vector, &bigram, positional_weight * 0.65);
                }
            }
        }

        l2_normalize(&mut vector);
        vector
    }
}
