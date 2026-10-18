//! Cached audit entries and cursor navigation.
use crate::AuditFilter;
use codetether_audit::AuditEntry;

/// State for the audit view: the cached entries plus UI cursor.
///
/// # Examples
/// ```
/// let state = codetether_audit_view::AuditViewState::default();
/// assert!(state.entries.is_empty());
/// ```
#[derive(Debug)]
pub struct AuditViewState {
    /// Entries in the most recent filtered snapshot.
    pub entries: Vec<AuditEntry>,
    /// Selected entry index.
    pub selected: usize,
    /// Categories shown by the view.
    pub filter: AuditFilter,
    /// Monotonic counter; bumps each refresh so the footer can show
    /// "updated N ticks ago" without pulling chrono into every frame.
    pub refresh_counter: u64,
}

impl Default for AuditViewState {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            selected: 0,
            filter: AuditFilter::Subagents,
            refresh_counter: 0,
        }
    }
}
