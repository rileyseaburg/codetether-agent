//! Scale guard for warm local recall ranking.

use chrono::Utc;

use crate::vectordb::LocalEmbeddingEngine;

#[test]
fn ranks_ten_thousand_documents_without_broad_loading() {
    let excerpt = "local recall evidence for deployment policy";
    let document = super::super::document::RecallDocument {
        start: 0,
        end: 4,
        excerpt: excerpt.into(),
        tokens: super::super::tokens::document(excerpt),
        embedding: LocalEmbeddingEngine::new(96).embed(excerpt),
    };
    let session = super::super::indexed_session::IndexedSession {
        schema_version: super::super::indexed_session::SCHEMA_VERSION,
        session_id: "scale-session".into(),
        title: None,
        workspace: "/tmp/work".into(),
        updated_at: Utc::now(),
        message_count: 40_000,
        documents: vec![document; 10_000],
    };
    let hits = super::super::rank::search(&[session], "deployment policy", None, 3, 0.2);
    assert_eq!(hits.len(), 3);
}
