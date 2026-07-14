//! Local hybrid-ranking tests.

use chrono::Utc;

use crate::vectordb::LocalEmbeddingEngine;

#[test]
fn lexical_evidence_is_ranked_with_provenance() {
    let excerpt = "[0 User]\nreplace session scan with local recall index";
    let document = super::super::document::RecallDocument {
        start: 0,
        end: 1,
        excerpt: excerpt.into(),
        tokens: super::super::tokens::document(excerpt),
        embedding: LocalEmbeddingEngine::new(96).embed(excerpt),
    };
    let session = super::super::indexed_session::IndexedSession {
        schema_version: super::super::indexed_session::SCHEMA_VERSION,
        session_id: "session-a".into(),
        title: Some("Recall work".into()),
        workspace: "/tmp/work".into(),
        updated_at: Utc::now(),
        message_count: 1,
        documents: vec![document],
    };
    let hits = super::super::rank::search(&[session], "local recall index", None, 3, 0.2);
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].session_id, "session-a");
    assert_eq!(hits[0].start, 0);
}
