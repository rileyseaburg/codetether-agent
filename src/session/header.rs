//! Lightweight header-only session parse for fast directory scanning.
//!
//! When searching for a session that matches a workspace, we only need the
//! `metadata.directory` field. This struct deserializes that plus a few
//! other small fields; serde_json still lexes past the heavy
//! `messages`/`tool_uses` arrays but never allocates a `Vec<Message>`,
//! which is the dominant cost for multi-megabyte session files.

use serde::Deserialize;

use super::types::SessionMetadata;

#[derive(Deserialize)]
pub(crate) struct SessionHeader {
    #[allow(dead_code)]
    pub id: String,
    #[allow(dead_code)]
    pub title: Option<String>,
    pub metadata: SessionMetadata,
}
