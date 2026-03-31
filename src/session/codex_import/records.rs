use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub(crate) struct CodexJsonlRecord {
    pub(crate) timestamp: DateTime<Utc>,
    #[serde(rename = "type")]
    pub(crate) kind: String,
    pub(crate) payload: Value,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CodexSessionMetaPayload {
    pub(crate) id: String,
    pub(crate) timestamp: DateTime<Utc>,
    pub(crate) cwd: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CodexTurnContextPayload {
    #[serde(default)]
    pub(crate) model: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CodexSessionIndexEntry {
    pub(crate) id: String,
    pub(crate) thread_name: String,
    pub(crate) updated_at: DateTime<Utc>,
}
