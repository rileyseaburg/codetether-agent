//! Compatible worker progress request; identity authorization is separate.

/// Worker payload accepted with optional compatibility fields.
/// # Examples
/// ```
/// let _request: codetether_task_api::output_ingest::TaskOutputPayload =
///     serde_json::from_str(r#"{}"#).unwrap();
/// ```
#[derive(serde::Deserialize)]
pub struct TaskOutputPayload {
    /// Retained for compatibility, not an authenticated worker identity.
    #[allow(dead_code)]
    #[serde(default)]
    pub worker_id: Option<String>,
    #[serde(default)]
    pub output: Option<String>,
}
