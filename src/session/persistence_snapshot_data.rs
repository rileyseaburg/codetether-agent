//! Borrowed serializable projection of a session.

use std::borrow::Cow;

use serde::Serialize;

use super::Session;

#[path = "persistence_snapshot_metadata.rs"]
mod metadata;

#[derive(Serialize)]
pub(super) struct Snapshot<'a> {
    id: &'a str,
    title: &'a Option<String>,
    created_at: &'a chrono::DateTime<chrono::Utc>,
    updated_at: &'a chrono::DateTime<chrono::Utc>,
    metadata: crate::session::SessionMetadata,
    agent: &'a str,
    messages: &'a [crate::provider::Message],
    pages: Cow<'a, [crate::session::pages::PageKind]>,
    summary_index: &'a crate::session::index::SummaryIndex,
    tool_uses: &'a [crate::agent::ToolUse],
    usage: &'a crate::provider::Usage,
}

impl<'a> Snapshot<'a> {
    pub(super) fn from_session(session: &'a Session) -> Self {
        let pages = if session.pages.len() == session.messages.len() {
            Cow::Borrowed(session.pages.as_slice())
        } else {
            Cow::Owned(crate::session::pages::classify_all(&session.messages))
        };
        Self {
            id: &session.id,
            title: &session.title,
            created_at: &session.created_at,
            updated_at: &session.updated_at,
            metadata: metadata::normalized(session),
            agent: &session.agent,
            messages: &session.messages,
            pages,
            summary_index: &session.summary_index,
            tool_uses: &session.tool_uses,
            usage: &session.usage,
        }
    }
}
