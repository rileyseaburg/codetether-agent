//! Import accounting and persistence outcomes.

/// Counts of storage outcomes from a workspace import.
/// # Examples
/// ```
/// let report = codetether_codex_import::CodexImportReport::default();
/// assert_eq!(report.imported + report.skipped, 0);
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CodexImportReport {
    /// Number of imports saved to native storage.
    pub imported: usize,
    /// Number of imports already present at an equal or newer version.
    pub skipped: usize,
}

/// Outcome of persisting one parsed import.
/// # Examples
/// ```
/// use codetether_codex_import::PersistOutcome;
/// let saved = match PersistOutcome::Saved {
///     PersistOutcome::Saved => true,
///     PersistOutcome::Unchanged => false,
/// };
/// assert!(saved);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersistOutcome {
    /// A newer imported session was saved.
    Saved,
    /// Existing storage was already current.
    Unchanged,
}
