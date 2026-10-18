/// Compact visible-state counters captured around one tick.
/// # Examples
/// ```
/// let first = codetether_tui_tick::Snapshot::default();
/// let mut next = first;
/// next.messages = 1;
/// assert_ne!(first, next);
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Snapshot {
    /// Number of visible chat messages.
    pub messages: usize,
    /// Input byte length.
    pub input: usize,
    /// Status byte length.
    pub status: usize,
    /// Streaming text byte length.
    pub streaming: usize,
    /// Number of bus-log entries.
    pub bus: usize,
    /// Audit refresh generation.
    pub audit_tick: u64,
    /// Number of queued worker tasks.
    pub queue: usize,
    /// Whether a request is being processed.
    pub processing: bool,
    /// Coarse animation clock for redraw throttling.
    pub animation_epoch: u64,
}
