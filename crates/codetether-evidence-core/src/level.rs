//! Evidence labels used in prompts, claims and response verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Evidence categories ordered from unexecuted work to platform proof.
/// # Examples
/// ```
/// use codetether_evidence_core::level::EvidenceLevel;
/// let level = EvidenceLevel::NotRun;
/// assert_eq!(level.label(), "not-run");
/// ```
pub enum EvidenceLevel {
    /// No execution evidence.
    NotRun,
    /// Static or local checks.
    StaticLocal,
    /// Local checks with mocked services.
    MockedLocal,
    /// A focused run approximating CI.
    FocusedCiLike,
    /// A live deployment or Argo observation.
    LiveDeploymentArgo,
    /// Evidence from an actual platform upload.
    RealPlatformUpload,
}

impl EvidenceLevel {
    /// Return the exact label used in prompts and verification claims.
    pub fn label(self) -> &'static str {
        match self {
            Self::NotRun => "not-run",
            Self::StaticLocal => "static/local",
            Self::MockedLocal => "mocked local",
            Self::FocusedCiLike => "focused CI-like",
            Self::LiveDeploymentArgo => "live deployment/Argo",
            Self::RealPlatformUpload => "real platform upload",
        }
    }

    /// Join all category labels in their established display order.
    pub fn all_labels() -> String {
        labels().join(", ")
    }
}

pub(super) fn labels() -> [&'static str; 6] {
    [
        EvidenceLevel::NotRun,
        EvidenceLevel::StaticLocal,
        EvidenceLevel::MockedLocal,
        EvidenceLevel::FocusedCiLike,
        EvidenceLevel::LiveDeploymentArgo,
        EvidenceLevel::RealPlatformUpload,
    ]
    .map(EvidenceLevel::label)
}
