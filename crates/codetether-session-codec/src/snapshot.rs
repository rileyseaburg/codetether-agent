//! Borrowed serializable projection of a session.

use std::borrow::Cow;

use serde::Serialize;

use crate::SnapshotView;

use crate::normalize as metadata;

#[derive(Serialize)]
pub(super) struct Snapshot<'a> {
    id: &'a str,
    title: &'a Option<String>,
    created_at: &'a chrono::DateTime<chrono::Utc>,
    updated_at: &'a chrono::DateTime<chrono::Utc>,
    metadata: codetether_session_metadata::SessionMetadata,
    agent: &'a str,
    messages: &'a [codetether_provider_core::Message],
    pages: Cow<'a, [codetether_context_policy::pages::PageKind]>,
    summary_index: &'a codetether_context_policy::summary_index::SummaryIndex,
    tool_uses: &'a [codetether_agent_core::ToolUse],
    usage: &'a codetether_provider_core::Usage,
}

impl<'a> Snapshot<'a> {
    pub(super) fn from_view(session: SnapshotView<'a>) -> Self {
        let pages = if session.pages.len() == session.messages.len() {
            Cow::Borrowed(session.pages)
        } else {
            Cow::Owned(codetether_context_policy::pages::classify_all(
                session.messages,
            ))
        };
        Self {
            id: session.header.id,
            title: session.header.title,
            created_at: session.header.created_at,
            updated_at: session.header.updated_at,
            metadata: metadata::normalized(session.header.metadata),
            agent: session.header.agent,
            messages: session.messages,
            pages,
            summary_index: session.summary_index,
            tool_uses: session.tool_uses,
            usage: session.header.usage,
        }
    }
}
