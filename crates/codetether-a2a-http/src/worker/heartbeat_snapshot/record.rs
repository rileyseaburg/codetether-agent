//! One heartbeat snapshot and its fallback payload.
/// Payloads produced from a worker heartbeat tick.
///
/// # Examples
/// ```rust
/// use codetether_a2a_http::worker::heartbeat_snapshot::HeartbeatPayload;
/// let tick = HeartbeatPayload {
///     payload: serde_json::json!({}), base_payload: serde_json::json!({}),
///     task_progress_payload: None, cognition_included: false,
/// };
/// assert!(!tick.cognition_included);
/// ```

pub struct HeartbeatPayload {
    /// Full heartbeat including optional cognition and task progress.
    pub payload: serde_json::Value,
    /// Base worker state used when cognition is rejected.
    pub base_payload: serde_json::Value,
    /// Active task progress for an optional extended heartbeat.
    pub task_progress_payload: Option<serde_json::Value>,
    /// Whether the full payload includes cognition metadata.
    pub cognition_included: bool,
}
