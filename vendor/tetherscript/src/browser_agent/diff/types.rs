/// Summary of coarse DOM differences.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::diff::diff_html;
///
/// let summary = diff_html("<p>old</p>", "<p>new</p>");
/// assert_eq!(summary.entries.len(), 1);
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DomDiffSummary {
    /// Ordered diff entries from a deterministic tree walk.
    pub entries: Vec<DomDiffEntry>,
}

impl DomDiffSummary {
    /// Returns true when the compared DOM trees have no reported changes.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// One coarse DOM difference at a stable path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomDiffEntry {
    /// Kind of change detected at `path`.
    pub kind: DomDiffKind,
    /// Index-based DOM path such as `/main[0]/p[1]/#text[0]`.
    pub path: String,
    /// Previous value, when the change has one.
    pub before: Option<String>,
    /// New value, when the change has one.
    pub after: Option<String>,
}

/// Coarse change categories reported by the DOM diff.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomDiffKind {
    /// A node exists only in the after tree.
    Inserted,
    /// A node exists only in the before tree.
    Removed,
    /// A text node changed value.
    TextChanged,
    /// An element kept its tag but changed attributes.
    AttributesChanged,
}
