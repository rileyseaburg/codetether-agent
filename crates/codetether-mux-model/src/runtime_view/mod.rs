//! Borrowed UI status projection and allocation-free change detection.
//!
//! [RuntimeView] borrows session/UI facts. [RuntimeView::unchanged] compares an
//! existing report; [RuntimeView::snapshot] allocates only for a changed report.
mod compare;
mod snapshot;
use codetether_provenance::RuntimePrincipal;
/// Borrowed facts used to report a running TUI to its mux owner.
/// # Examples
/// Native frontends project their current state into this view before reporting.
pub struct RuntimeView<'a> {
    /// Durable active session identity.
    pub session_id: &'a str,
    /// Optional display title.
    pub title: Option<&'a str>,
    /// Whether a turn is being processed.
    pub processing: bool,
    /// Durable transcript message count.
    pub message_count: usize,
    /// Current tool name when one is pending.
    pub current_tool: Option<&'a str>,
    /// Current editable input, used to detect requested interaction.
    pub input: &'a str,
    /// Whether a watchdog notice is pending.
    pub watchdog_pending: bool,
    /// Automatic restart attempts since the last user prompt.
    pub restarts: u32,
    /// Current visible status, including terminal watchdog errors.
    pub status: &'a str,
    /// Session provenance reported outside conversational output.
    pub principal: &'a RuntimePrincipal,
}
impl RuntimeView<'_> {
    fn lagging(&self) -> bool {
        self.watchdog_pending
            || (self.processing && self.restarts > 0)
            || self.status.starts_with("Watchdog gave up")
    }
}
