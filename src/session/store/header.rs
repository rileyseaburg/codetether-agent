//! Mutable headers never include transcript or summary payloads.
use crate::session::Session;
use anyhow::Result;
pub(super) fn encode(session: &Session) -> Result<String> {
    let mut metadata = session.metadata.clone();
    if let Some(directory) = &mut metadata.directory {
        *directory = directory
            .canonicalize()
            .unwrap_or_else(|_| directory.clone());
    }
    if let Some(identity) = crate::provenance::runtime_agent_identity()
        && let Some(provenance) = metadata.provenance.as_mut()
    {
        provenance.identity.agent_identity_id = Some(identity);
    }
    write(session, &metadata)
}
pub(super) fn legacy(session: &Session) -> Result<String> {
    write(session, &session.metadata)
}
fn write(session: &Session, metadata: &crate::session::SessionMetadata) -> Result<String> {
    Ok(serde_json::to_string(&serde_json::json!({
        "id": session.id, "title": session.title,
        "created_at": session.created_at, "updated_at": session.updated_at,
        "metadata": metadata, "agent": session.agent,
        "usage": session.usage,
        "messages": [], "pages": [], "tool_uses": []
    }))?)
}
pub(super) fn decode(header: &str) -> Result<Session> {
    let session = serde_json::from_str(header)?;
    Ok(session)
}
