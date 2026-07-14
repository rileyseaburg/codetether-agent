//! Build compact recall projections from in-memory sessions.

use crate::session::Session;
use crate::vectordb::LocalEmbeddingEngine;

use super::document::RecallDocument;
use super::indexed_session::{IndexedSession, SCHEMA_VERSION};

const CHUNK_MESSAGES: usize = 4;
const MAX_DOCUMENTS: usize = 128;
const EMBEDDING_DIMENSIONS: usize = 96;

pub(super) fn session(session: &Session) -> Option<IndexedSession> {
    let workspace = super::paths::canonical(session.metadata.directory.as_deref()?);
    let chunks = session.messages.chunks(CHUNK_MESSAGES);
    let skip = chunks.len().saturating_sub(MAX_DOCUMENTS);
    let engine = LocalEmbeddingEngine::new(EMBEDDING_DIMENSIONS);
    let documents = session
        .messages
        .chunks(CHUNK_MESSAGES)
        .enumerate()
        .skip(skip)
        .map(|(index, messages)| document(index, messages, &engine))
        .collect();
    Some(IndexedSession {
        schema_version: SCHEMA_VERSION,
        session_id: session.id.clone(),
        title: session.title.clone(),
        workspace,
        updated_at: session.updated_at,
        message_count: session.messages.len(),
        documents,
    })
}

fn document(
    index: usize,
    messages: &[crate::provider::Message],
    engine: &LocalEmbeddingEngine,
) -> RecallDocument {
    let start = index * CHUNK_MESSAGES;
    let excerpt = super::excerpt::render(messages, start);
    RecallDocument {
        start,
        end: start + messages.len(),
        tokens: super::tokens::document(&excerpt),
        embedding: engine.embed(&excerpt),
        excerpt,
    }
}
