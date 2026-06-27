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

#[tokio::test]
async fn search_embedded_falls_back_to_local_without_backend() {
    let mut store = MemoryStore::default();
    store.add(embedded("rust async tokio runtime tasks"));
    store.add(embedded("python data science pandas"));

    let results = store
        .search_embedded(Some("tokio runtime"), None, None, 5)
        .await;
    assert!(results[0].content.contains("tokio"));
}
