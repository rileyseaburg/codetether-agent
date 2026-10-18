//! Session row fixtures.
use chrono::Utc;
use codetether_session_metadata::SessionSummary;
use std::path::PathBuf;

pub(crate) fn summary(title: Option<&str>, dir: Option<&str>) -> SessionSummary {
    SessionSummary {
        id: "abcdef1234567890".to_string(),
        title: title.map(str::to_string),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        message_count: 7,
        agent: "default".to_string(),
        directory: dir.map(PathBuf::from),
    }
}
