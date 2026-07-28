//! Current page guard metadata.

/// Snapshot of resource usage compared with page limits.
///
/// # Examples
///
/// ```rust
/// use tetherscript::browser_agent::BrowserPage;
///
/// let page = BrowserPage::from_html("mem://limits", "<main></main>");
/// let metadata = page.guard_metadata();
///
/// assert!(metadata.dom_bytes <= metadata.max_dom_bytes);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BrowserGuardMetadata {
    /// Current serialized DOM size in bytes.
    pub dom_bytes: usize,
    /// Current retained action trace entry count.
    pub trace_entries: usize,
    /// Configured DOM byte limit.
    pub max_dom_bytes: usize,
    /// Configured trace entry limit.
    pub max_trace_entries: usize,
    /// Configured action attempt limit.
    pub max_action_attempts: usize,
    /// Configured action tick limit.
    pub max_action_ticks: usize,
    /// Configured raster pixel metadata limit.
    pub max_raster_pixels: usize,
}
