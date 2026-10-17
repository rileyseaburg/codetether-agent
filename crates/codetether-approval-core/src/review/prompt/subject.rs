//! Description of the action submitted for review.

/// Inputs gathered from the approval request and the session.
///
/// # Examples
/// ```
/// use codetether_approval_core::review::ReviewSubject;
/// let subject = ReviewSubject { tool: "write".into(), ..Default::default() };
/// assert_eq!(subject.tool, "write");
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReviewSubject {
    /// Name of the proposed tool.
    pub tool: String,
    /// Action the tool intends to perform.
    pub action: String,
    /// Resource affected by the action.
    pub resource: String,
    /// Author's explanation for the proposed action.
    pub justification: Option<String>,
    /// Optional rendered preview of the change.
    pub preview: Option<String>,
    /// Rendered goal governance block, when the session has a goal.
    pub goal: Option<String>,
}
