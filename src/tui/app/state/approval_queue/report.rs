//! Real language-server analysis shown with an approval preview.

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ApprovalReport {
    pub(crate) state: ApprovalReportState,
    pub(crate) messages: Vec<String>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum ApprovalReportState {
    #[default]
    NotRequested,
    Checking,
    Clean,
    Issues,
    Unavailable,
}

impl ApprovalReport {
    pub(crate) fn checking() -> Self {
        Self {
            state: ApprovalReportState::Checking,
            messages: Vec::new(),
        }
    }
}
