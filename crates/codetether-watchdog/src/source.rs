//! Read access used by stall detection without owning application state.
use codetether_tool_tracker::ToolCallTracker;
use std::time::Instant;
/// Host state required by the watchdog detector.
///
/// # Implementors
/// Implement for the UI state. Keep human-wait checks in skip_stall_check so
/// storage and question runtimes are only queried at the original early gate.
pub trait WatchdogSource {
    /// Whether processing is paused, already notified, or waiting for a human.
    fn skip_stall_check(&self) -> bool;
    /// Current automatic restart count.
    fn restart_count(&self) -> u32;
    /// Current request start clock.
    fn request_started_at(&self) -> Option<Instant>;
    /// Current request first-token latency, if a token arrived.
    fn first_token_ms(&self) -> Option<u64>;
    /// Last observed session activity clock.
    fn last_event_at(&self) -> Option<Instant>;
    /// Bus-observed open tool calls and their runtime budgets.
    fn tool_calls(&self) -> &ToolCallTracker;
}
