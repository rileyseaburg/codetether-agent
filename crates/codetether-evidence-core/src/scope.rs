//! Scope-completion labels used in evidence prompts and ledgers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Reported state of one requested deliverable.
/// # Examples
/// ```
/// assert_eq!(codetether_evidence_core::scope::ScopeStatus::Pending.label(), "pending");
/// ```
pub enum ScopeStatus {
    /// Evidence establishes completion.
    Proven,
    /// Work remains outstanding.
    Pending,
    /// A concrete blocker prevents progress.
    Blocked,
    /// Verification or execution has not occurred.
    NotRun,
}

impl ScopeStatus {
    /// Return the scope status's exact reporting label.
    pub fn label(self) -> &'static str {
        match self {
            Self::Proven => "proven",
            Self::Pending => "pending",
            Self::Blocked => "blocked",
            Self::NotRun => "not-run",
        }
    }
}

/// Join scope status labels in their established reporting order.
/// # Arguments
/// This function takes no arguments.
/// # Returns
/// The comma-separated proven, pending, blocked and not-run labels.
/// # Examples
/// ```
/// assert!(codetether_evidence_core::scope::status_labels().contains("not-run"));
/// ```
pub fn status_labels() -> String {
    [
        ScopeStatus::Proven,
        ScopeStatus::Pending,
        ScopeStatus::Blocked,
        ScopeStatus::NotRun,
    ]
    .map(ScopeStatus::label)
    .join(", ")
}
