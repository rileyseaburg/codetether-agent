//! Real language-server analysis shown with an approval preview.

/// Analysis state and diagnostic messages for an approval preview.
/// # Examples
/// ```
/// assert!(codetether_approval_queue::ApprovalReport::default().messages.is_empty());
/// ```
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ApprovalReport {
    /// Current analysis phase.
    pub state: ApprovalReportState,
    /// Human-readable diagnostics.
    pub messages: Vec<String>,
}

/// Progress of source analysis for an approval preview.
/// # Examples
/// ```
/// assert_eq!(codetether_approval_queue::ApprovalReportState::default(), codetether_approval_queue::ApprovalReportState::NotRequested);
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ApprovalReportState {
    #[default]
    /// No analysis requested.
    NotRequested,
    /// Analysis is running.
    Checking,
    /// Analysis completed without diagnostics.
    Clean,
    /// Analysis found diagnostics.
    Issues,
    /// Analysis could not run.
    Unavailable,
}

impl ApprovalReport {
    /// Construct a report marking analysis as running.
    pub fn checking() -> Self {
        Self {
            state: ApprovalReportState::Checking,
            messages: Vec::new(),
        }
    }
}
