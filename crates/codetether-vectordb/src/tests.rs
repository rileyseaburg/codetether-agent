//! Unit tests for the vector store.

use super::*;

#[test]
fn upsert_replaces_existing_id() {
    let engine = LocalEmbeddingEngine::new(32);
    let mut store: VectorStore<String> = VectorStore::new();
    store.upsert("x", engine.embed("first"), "first".into());
    store.upsert("x", engine.embed("second"), "second".into());
    assert_eq!(store.len(), 1);
    assert_eq!(store.records()[0].payload, "second");
}

#[test]
fn search_ranks_by_semantic_similarity() {
    let engine = LocalEmbeddingEngine::new(128);
    let mut store: VectorStore<String> = VectorStore::new();
    store.upsert(
        "rust",
        engine.embed("rust async tokio runtime"),
        "rust".into(),
    );
    store.upsert(
        "k8s",
        engine.embed("kubernetes pod deployment scaling"),
        "k8s".into(),
    );

    let hits = store.search(&engine.embed("tokio async runtime"), 2);
    assert_eq!(hits[0].record.id, "rust");
}

#[test]
fn remove_deletes_record() {
    let engine = LocalEmbeddingEngine::new(16);
    let mut store: VectorStore<String> = VectorStore::new();
    store.upsert("a", engine.embed("a"), "a".into());
    assert!(store.remove("a"));
    assert!(!store.remove("a"));
    assert!(store.is_empty());
}

#[test]
fn persistence_round_trips() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("vectors.json");
    let engine = LocalEmbeddingEngine::new(32);
    {
        let mut store: VectorStore<String> = VectorStore::open(&path).unwrap();
        store.upsert("a", engine.embed("hello"), "hello".into());
        store.save().unwrap();
    }
    let reopened: VectorStore<String> = VectorStore::open(&path).unwrap();
    assert_eq!(reopened.len(), 1);
    assert_eq!(reopened.records()[0].payload, "hello");
}
