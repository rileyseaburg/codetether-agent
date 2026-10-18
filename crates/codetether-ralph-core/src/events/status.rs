//! RalphStoryStatus contract.

// ---------------------------------------------------------------------------
// Story status
// ---------------------------------------------------------------------------

/// Status of an individual story in the Ralph view
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
///
/// # Examples
/// ```
/// use codetether_ralph_core::events::RalphStoryStatus;
/// let status = RalphStoryStatus::Pending;
/// match status { RalphStoryStatus::Pending => {}, RalphStoryStatus::Blocked => {}, RalphStoryStatus::Running => {}, RalphStoryStatus::QualityCheck => {}, RalphStoryStatus::Passed => {}, RalphStoryStatus::Failed => {} }
/// ```
pub enum RalphStoryStatus {
    Pending,
    Blocked,
    Running,
    QualityCheck,
    Passed,
    Failed,
}
