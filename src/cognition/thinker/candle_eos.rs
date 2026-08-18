//! End-of-sequence token resolution for Candle generation.

use std::collections::HashSet;
use tokenizers::Tokenizer;

/// Well-known EOS/EOT strings used when GGUF metadata is incomplete.
const CANDIDATES: &[&str] = &[
    "<|im_end|>",
    "<|eot_id|>",
    "<|endoftext|>",
    "</s>",
    "<|end|>",
    "<end_of_turn>",
];

/// Combine GGUF-declared EOS IDs with tokenizer lookups of known EOS strings.
pub(super) fn collect_eos_token_ids(tokenizer: &Tokenizer, gguf_eos_ids: &[u32]) -> HashSet<u32> {
    let mut ids: HashSet<u32> = gguf_eos_ids.iter().copied().collect();
    for token in CANDIDATES {
        if let Some(id) = tokenizer.token_to_id(token) {
            ids.insert(id);
        }
    }
    ids
}
