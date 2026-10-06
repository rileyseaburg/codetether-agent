//! Compatible worker progress request; identity authorization is separate.

#[derive(serde::Deserialize)]
pub(in crate::server) struct TaskOutputPayload {
    /// Retained for compatibility, not an authenticated worker identity.
    #[allow(dead_code)]
    #[serde(default)]
    pub worker_id: Option<String>,
    #[serde(default)]
    pub output: Option<String>,
}
