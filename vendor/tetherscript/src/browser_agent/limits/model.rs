//! Resource limit model for browser pages.

/// Deterministic resource budgets for one browser page.
///
/// # Examples
///
/// ```rust
/// use tetherscript::browser_agent::BrowserResourceLimits;
///
/// let limits = BrowserResourceLimits::default();
/// assert!(limits.max_dom_bytes > 0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BrowserResourceLimits {
    /// Maximum retry attempts an action may spend.
    pub max_action_attempts: usize,
    /// Maximum deterministic ticks an action may spend.
    pub max_action_ticks: usize,
    /// Maximum serialized DOM size allowed before runtime work.
    pub max_dom_bytes: usize,
    /// Maximum retained action trace entries.
    pub max_trace_entries: usize,
    /// Maximum raster output pixels accepted by page metadata.
    pub max_raster_pixels: usize,
}

impl Default for BrowserResourceLimits {
    fn default() -> Self {
        Self {
            max_action_attempts: 9,
            max_action_ticks: 8,
            max_dom_bytes: 1_000_000,
            max_trace_entries: 1_000,
            max_raster_pixels: 8_000_000,
        }
    }
}

impl BrowserResourceLimits {
    pub(crate) fn retry_ticks(self) -> usize {
        self.max_action_ticks
            .min(self.max_action_attempts.saturating_sub(1))
    }
}
