//! Indexer behavior regression.

use super::*;

#[test]
fn local_embeddings_have_expected_dimensions() {
    let engine = LocalEmbeddingEngine::new(384);
    let vectors = engine.embed_batch(&["fn main() { println!(\"hi\") }".to_string()]);
    assert_eq!(vectors.len(), 1);
    assert_eq!(vectors[0].len(), 384);
}
