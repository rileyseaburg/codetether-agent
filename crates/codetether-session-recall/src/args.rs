//! Typed parsing for session-recall tool arguments.

use serde_json::Value;

const DEFAULT_LIMIT: usize = 3;
const MAX_LIMIT: usize = 5;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum RecallMode {
    Evidence,
    Answer,
}

pub(super) struct RecallArgs {
    pub query: String,
    pub session_id: Option<String>,
    pub limit: usize,
    pub mode: RecallMode,
}

impl RecallArgs {
    pub(super) fn parse(value: &Value) -> Result<Self, String> {
        let query = value
            .get("query")
            .and_then(Value::as_str)
            .filter(|query| !query.trim().is_empty())
            .ok_or_else(|| "query is required".to_string())?;
        let mode = match value.get("mode").and_then(Value::as_str) {
            None | Some("evidence") => RecallMode::Evidence,
            Some("answer") => RecallMode::Answer,
            Some(other) => return Err(format!("unknown recall mode: {other}")),
        };
        Ok(Self {
            query: query.to_string(),
            session_id: value
                .get("session_id")
                .and_then(Value::as_str)
                .map(str::to_string),
            limit: value
                .get("limit")
                .and_then(Value::as_u64)
                .map(|limit| limit as usize)
                .unwrap_or(DEFAULT_LIMIT)
                .clamp(1, MAX_LIMIT),
            mode,
        })
    }
}
