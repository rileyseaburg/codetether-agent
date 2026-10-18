//! Accumulated timing data for one tool name.

#[derive(Debug, Default)]
pub(crate) struct ToolLatencySummary {
    pub(super) count: usize,
    pub(super) failures: usize,
    pub(super) total_ms: u64,
    pub(super) max_ms: u64,
    pub(super) last_ms: u64,
}
