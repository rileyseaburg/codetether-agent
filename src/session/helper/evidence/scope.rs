#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ScopeStatus {
    Proven,
    Pending,
    Blocked,
    NotRun,
}

impl ScopeStatus {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Proven => "proven",
            Self::Pending => "pending",
            Self::Blocked => "blocked",
            Self::NotRun => "not-run",
        }
    }
}

pub(crate) fn status_labels() -> String {
    [
        ScopeStatus::Proven,
        ScopeStatus::Pending,
        ScopeStatus::Blocked,
        ScopeStatus::NotRun,
    ]
    .map(ScopeStatus::label)
    .join(", ")
}
