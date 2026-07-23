//! State carried across a delayed smart-switch retry.

/// A pending smart-switch retry with optional temporary-failover metadata.
#[derive(Debug, Clone)]
pub struct PendingSmartSwitchRetry {
    /// Original prompt to submit after the retry gate opens.
    pub prompt: String,
    /// Model selected for this retry or restoration.
    pub target_model: String,
    /// Earliest instant at which this retry may run.
    pub not_before: Option<std::time::Instant>,
    /// Fallback model currently being used while restoration is pending.
    pub restore_from: Option<String>,
    /// Deadline after which the original model may be restored.
    pub restore_at: Option<std::time::Instant>,
    /// Original model to restore after the fallback completes.
    pub restore_model: Option<String>,
}
