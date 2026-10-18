//! Build compact recall projections from in-memory sessions.

use crate::ProjectionInput;
use codetether_vectordb::LocalEmbeddingEngine;

use super::build_document::document;
use super::indexed_session::{IndexedSession, SCHEMA_VERSION};

const CHUNK_MESSAGES: usize = 4;
const MAX_DOCUMENTS: usize = 128;
const EMBEDDING_DIMENSIONS: usize = 96;

/// Build the newest bounded evidence windows from a loaded transcript.
/// # Arguments
/// * `session` - Transcript window and absolute provenance from the caller.
/// # Returns
/// A serializable projection containing at most 128 four-message documents.
/// # Examples
/// ```
/// use codetether_recall_index::{project, ProjectionInput};
/// let result = project(ProjectionInput { messages: &[], session_id: "id",
///     title: None, workspace: "/tmp".into(), updated_at: chrono::Utc::now(),
///     message_offset: 0, message_count: 0 });
/// assert!(result.is_current_schema());
/// ```
pub fn project(session: ProjectionInput<'_>) -> IndexedSession {
    let workspace = session.workspace;
    let chunks = session.messages.chunks(CHUNK_MESSAGES);
    let skip = chunks.len().saturating_sub(MAX_DOCUMENTS);
    let engine = LocalEmbeddingEngine::new(EMBEDDING_DIMENSIONS);
    let documents = session
        .messages
        .chunks(CHUNK_MESSAGES)
        .enumerate()
        .skip(skip)
        .map(|(index, messages)| {
            document(
                index * CHUNK_MESSAGES + session.message_offset,
                messages,
                &engine,
            )
        })
        .collect();
    IndexedSession {
        schema_version: SCHEMA_VERSION,
        session_id: session.session_id.to_string(),
        title: session.title.map(str::to_string),
        workspace,
        updated_at: session.updated_at,
        message_count: session.message_count,
        documents,
    }
}
