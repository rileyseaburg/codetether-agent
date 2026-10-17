//! Expose current execution state and shared status rendering.
use super::*;
impl RalphLoop {
    /// Get current status
    pub fn status(&self) -> &RalphState {
        &self.state
    }

    /// Format status as markdown
    pub fn status_markdown(&self) -> String {
        codetether_ralph_core::presentation::status_markdown(&self.state)
    }
}
