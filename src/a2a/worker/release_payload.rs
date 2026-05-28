//! Worker task release payload.

use serde::Serialize;

#[derive(Serialize)]
pub(super) struct Payload<'a> {
    pub task_id: &'a str,
    pub status: &'a str,
    pub result: Option<String>,
    pub error: Option<String>,
    pub session_id: Option<String>,
    pub diagnostics: Option<serde_json::Value>,
}
