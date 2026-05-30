//! Static list of all task checkpoints.

use super::TaskCheckpoint;

impl TaskCheckpoint {
    #[allow(dead_code)]
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
