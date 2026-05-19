#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EvidenceLevel {
    NotRun,
    StaticLocal,
    MockedLocal,
    FocusedCiLike,
    LiveDeploymentArgo,
    RealPlatformUpload,
}

impl EvidenceLevel {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::NotRun => "not-run",
            Self::StaticLocal => "static/local",
            Self::MockedLocal => "mocked local",
            Self::FocusedCiLike => "focused CI-like",
            Self::LiveDeploymentArgo => "live deployment/Argo",
            Self::RealPlatformUpload => "real platform upload",
        }
    }

    pub(crate) fn all_labels() -> String {
        [
            Self::NotRun,
            Self::StaticLocal,
            Self::MockedLocal,
            Self::FocusedCiLike,
            Self::LiveDeploymentArgo,
            Self::RealPlatformUpload,
        ]
        .map(Self::label)
        .join(", ")
    }
}
