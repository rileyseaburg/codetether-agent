//! Wait timeout options for deterministic agent page actions.

use crate::browser_agent::page::BrowserPage;

/// Retry budget for waits and retryable actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaitOptions {
    /// Number of deterministic page-settle ticks before timing out.
    pub timeout_ticks: usize,
}

impl Default for WaitOptions {
    fn default() -> Self {
        Self { timeout_ticks: 8 }
    }
}

impl BrowserPage {
    /// Set the page default retry budget for waits and retryable actions.
    pub fn set_default_timeout_ticks(&mut self, timeout_ticks: usize) {
        self.wait_options.timeout_ticks = timeout_ticks;
    }
}
