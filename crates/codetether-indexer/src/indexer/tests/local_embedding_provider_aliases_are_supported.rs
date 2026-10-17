//! Indexer behavior regression.

use super::*;

#[test]
fn local_embedding_provider_aliases_are_supported() {
    assert!(is_local_embedding_provider("local"));
    assert!(is_local_embedding_provider("local/hash-embedding"));
    assert!(is_local_embedding_provider("HASH"));
    assert!(!is_local_embedding_provider("huggingface"));
}
