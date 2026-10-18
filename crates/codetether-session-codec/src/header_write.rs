//! The compact persisted header JSON shape, excluding history payloads.
use crate::HeaderView;
use anyhow::Result;
use codetether_session_metadata::SessionMetadata;

pub(super) fn write(session: &HeaderView<'_>, metadata: &SessionMetadata) -> Result<String> {
    Ok(serde_json::to_string(&serde_json::json!({
        "id": session.id, "title": session.title,
        "created_at": session.created_at, "updated_at": session.updated_at,
        "metadata": metadata, "agent": session.agent,
        "usage": session.usage,
        "messages": [], "pages": [], "tool_uses": []
    }))?)
}
