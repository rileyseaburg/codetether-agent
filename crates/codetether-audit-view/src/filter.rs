//! Audit category selection.
use codetether_audit::{AuditCategory, AuditEntry};

/// Which slice of the audit log the view is currently showing.
///
/// # Examples
/// ```
/// use codetether_audit_view::AuditFilter;
/// assert_eq!(match AuditFilter::Subagents {
///     AuditFilter::Subagents => "Subagents",
///     AuditFilter::All => "All",
/// }, "Subagents");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditFilter {
    /// Subagent-relevant only: Swarm + ToolExecution + Cognition.
    Subagents,
    /// Every category.
    All,
}

impl AuditFilter {
    pub(crate) fn matches(self, e: &AuditEntry) -> bool {
        match self {
            Self::All => true,
            Self::Subagents => matches!(
                e.category,
                AuditCategory::Swarm | AuditCategory::ToolExecution | AuditCategory::Cognition
            ),
        }
    }

    /// Short label for the view header.
    pub fn label(self) -> &'static str {
        match self {
            Self::Subagents => "Subagents",
            Self::All => "All",
        }
    }
}
