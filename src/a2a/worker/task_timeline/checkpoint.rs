/// Named checkpoints along the task processing pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskCheckpoint {
    TaskReceived,
    SlotReserved,
    ClaimRequested,
    Claimed,
    MetadataParsed,
    WorkspaceReady,
    SessionReady,
    GitHookInstalled,
    ProvidersLoaded,
    ModelSelected,
    AgentStarting,
    AgentRunning,
    AgentDone,
    SessionSaved,
    CommitStaging,
    CommitCreated,
    CommitPushing,
    CommitPushed,
    PrCreating,
    PrCreated,
    Releasing,
    Released,
    Completed,
    GracefulShutdown,
    Failed,
}

impl TaskCheckpoint {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TaskReceived => "task_received",
            Self::SlotReserved => "slot_reserved",
            Self::ClaimRequested => "claim_requested",
            Self::Claimed => "claimed",
            Self::MetadataParsed => "metadata_parsed",
            Self::WorkspaceReady => "workspace_ready",
            Self::SessionReady => "session_ready",
            Self::GitHookInstalled => "git_hook_installed",
            Self::ProvidersLoaded => "providers_loaded",
            Self::ModelSelected => "model_selected",
            Self::AgentStarting => "agent_starting",
            Self::AgentRunning => "agent_running",
            Self::AgentDone => "agent_done",
            Self::SessionSaved => "session_saved",
            Self::CommitStaging => "commit_staging",
            Self::CommitCreated => "commit_created",
            Self::CommitPushing => "commit_pushing",
            Self::CommitPushed => "commit_pushed",
            Self::PrCreating => "pr_creating",
            Self::PrCreated => "pr_created",
            Self::Releasing => "releasing",
            Self::Released => "released",
            Self::Completed => "completed",
            Self::GracefulShutdown => "graceful_shutdown",
            Self::Failed => "failed",
        }
    }

    pub const ALL: &[TaskCheckpoint] = &[
        Self::TaskReceived,
        Self::SlotReserved,
        Self::ClaimRequested,
        Self::Claimed,
        Self::MetadataParsed,
        Self::WorkspaceReady,
        Self::SessionReady,
        Self::GitHookInstalled,
        Self::ProvidersLoaded,
        Self::ModelSelected,
        Self::AgentStarting,
        Self::AgentRunning,
        Self::AgentDone,
        Self::SessionSaved,
        Self::CommitStaging,
        Self::CommitCreated,
        Self::CommitPushing,
        Self::CommitPushed,
        Self::PrCreating,
        Self::PrCreated,
        Self::Releasing,
        Self::Released,
        Self::Completed,
        Self::GracefulShutdown,
        Self::Failed,
    ];
}

impl std::fmt::Display for TaskCheckpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
