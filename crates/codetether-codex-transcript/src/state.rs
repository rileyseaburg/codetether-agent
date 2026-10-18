//! Mutable fields accumulated while streaming a Codex transcript.
use crate::budget::BoundedMessages;
use chrono::{DateTime, Utc};
use codetether_codex_records::CodexSessionMetaPayload;
use codetether_provider_core::Usage;
pub(crate) struct State {
    pub(crate) session_meta: Option<CodexSessionMetaPayload>,
    pub(crate) updated_at: Option<DateTime<Utc>>,
    pub(crate) messages: BoundedMessages,
    pub(crate) usage: Usage,
    pub(crate) model: Option<String>,
    pub(crate) first_user_text: Option<String>,
}
