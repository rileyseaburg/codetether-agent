#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RuntimeCapability {
    ParallelScheduler,
    SpeculativePrefetch,
    EvidenceClassifier,
    BackgroundContextIndex,
    ToolOutputDigest,
    DynamicTetherScript,
    EventBusWorkflow,
    TypedWorkflowGate,
    EndToEndScopeGate,
}

impl RuntimeCapability {
    pub(crate) fn instruction(self) -> &'static str {
        match self {
            Self::ParallelScheduler => "Batch independent read-only tools; serialize mutations.",
            Self::SpeculativePrefetch => "Prefetch obvious files, Argo state, logs, and artifacts.",
            Self::EvidenceClassifier => "Classify proof as local, mocked, live, or platform.",
            Self::BackgroundContextIndex => {
                "Use indexed session, git, artifact, and cluster facts."
            }
            Self::ToolOutputDigest => "Return compact digests while preserving full artifacts.",
            Self::DynamicTetherScript => "Use TetherScript for repeatable parsers and validators.",
            Self::EventBusWorkflow => "Stream partial evidence through the session event bus.",
            Self::TypedWorkflowGate => "Do not finish until the typed workflow gate is satisfied.",
            Self::EndToEndScopeGate => "Track every user deliverable until proven or blocked.",
        }
    }
}
