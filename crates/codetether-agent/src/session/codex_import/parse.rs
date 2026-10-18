//! Construct native sessions from the shared bounded transcript parser.
use super::super::{Session, SessionMetadata};
use crate::provenance::ExecutionProvenance;
use anyhow::Result;
use std::path::Path;
pub(crate) fn parse_codex_session_from_path(
    path: &Path,
    title_override: Option<&str>,
) -> Result<Session> {
    let parsed = codetether_codex_transcript::parse_codex_session_from_path(path, title_override)?;
    let id = parsed.id;
    Ok(Session {
        id: id.clone(),
        title: parsed.title,
        created_at: parsed.created_at,
        updated_at: parsed.updated_at,
        messages: parsed.messages.into(),
        pages: Default::default(),
        summary_index: crate::session::index::SummaryIndex::new(),
        tool_uses: Default::default(),
        usage: parsed.usage,
        agent: "build".to_string(),
        metadata: SessionMetadata {
            directory: parsed.directory,
            model: parsed.model,
            provenance: Some(ExecutionProvenance::for_session(&id, "build")),
            ..Default::default()
        },
        max_steps: None,
        ..Default::default()
    })
}
