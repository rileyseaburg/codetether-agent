//! Tests for memory [`search`](super) ranking.

use super::super::{MemoryEntry, MemoryStore};

fn embedded(content: &str) -> MemoryEntry {
    let mut e = MemoryEntry::new(content, vec![]);
    e.ensure_embedding();
    e
}

#[test]
fn ranks_semantically_closest_first() {
    let mut store = MemoryStore::default();
    // Both share token "runtime" so both pass the lexical filter; the
    // semantically closer tokio entry should win after fusion.
    store.add(embedded("java jvm runtime garbage collection"));
    store.add(embedded("rust async tokio runtime tasks"));

    let results = store.search(Some("tokio async runtime"), None, None, 2);
    assert_eq!(results.len(), 2);
    assert!(results[0].content.contains("tokio"));
}

#[test]
fn refresh_embedding_replaces_unknown_vector_space() {
    let mut entry = MemoryEntry::new("local vector", vec![]);
    entry.embedding = Some(crate::vectordb::EmbeddingVector::new(vec![1.0, 0.0]));
    entry.refresh_embedding();
    assert_ne!(entry.embedding.as_ref().expect("embedding").len(), 2);
}

#[tokio::test]
async fn search_embedded_without_backend_uses_lexical_ranking() {
    // No embedder configured: query_vec is None, ranking is lexical-only.
    // The tokio entry shares more tokens with the query so it ranks first.
    // Critically: we do NOT mix hash-space vectors with learned-space vectors.
    let mut store = MemoryStore::default();
    store.add(embedded("rust async tokio runtime tasks"));
    store.add(embedded("python data science pandas"));

    let results = store
        .search_embedded(Some("tokio runtime"), None, None, 5)
        .await;
    assert!(!results.is_empty());
    assert!(results[0].content.contains("tokio"));
}
