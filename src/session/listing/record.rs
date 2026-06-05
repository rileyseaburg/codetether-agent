use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::count_seq::CountSeq;
use super::summary::SessionSummary;
use crate::session::SessionMetadata;

#[derive(Deserialize)]
pub(super) struct SessionListingRecord {
    id: String,
    title: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    metadata: SessionMetadata,
    agent: String,
    #[serde(default)]
    messages: CountSeq,
}

impl SessionListingRecord {
    pub(super) fn into_summary(self) -> SessionSummary {
        SessionSummary {
            id: self.id,
            title: self.title,
            created_at: self.created_at,
            updated_at: self.updated_at,
            message_count: self.messages.0,
            agent: self.agent,
            directory: self.metadata.directory,
        }
    }
}
