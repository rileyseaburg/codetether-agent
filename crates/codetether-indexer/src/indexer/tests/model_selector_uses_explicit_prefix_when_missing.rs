//! Indexer behavior regression.

use super::*;

#[test]
fn model_selector_uses_explicit_prefix_when_missing() {
    let selector = build_model_selector("huggingface", "BAAI/bge-small-en-v1.5")
        .expect("model selector should build");
    assert_eq!(selector, "BAAI/bge-small-en-v1.5");

    let selector = build_model_selector("huggingface", "text-embedding-3-large")
        .expect("model selector should build");
    assert_eq!(selector, "huggingface/text-embedding-3-large");
}
