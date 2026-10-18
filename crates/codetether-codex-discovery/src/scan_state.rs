//! Summary fields accumulated while streaming an archive.
use chrono::{DateTime, Utc};

pub(crate) struct CodexSummaryScan {
    pub(crate) updated_at: Option<DateTime<Utc>>,
    pub(crate) message_count: usize,
    pub(crate) first_user_text: Option<String>,
}
