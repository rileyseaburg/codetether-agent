//! Serializable thread lifecycle values returned by collaboration tools.

use serde::{Deserialize, Serialize};

/// Current lifecycle state for one child thread.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ThreadStatus {
    /// The child exists but its first turn has not started.
    #[default]
    PendingInit,
    /// A turn is currently executing.
    Running,
    /// The active turn was interrupted and the child accepts more input.
    Interrupted,
    /// The latest turn completed, optionally with its final answer.
    Completed(Option<String>),
    /// The latest turn failed.
    Errored(String),
    /// The child was closed but can be resumed.
    Shutdown,
    /// The child does not exist.
    NotFound,
}

impl ThreadStatus {
    /// Whether waiting callers should stop waiting for this status.
    pub(crate) fn is_final(&self) -> bool {
        !matches!(self, Self::PendingInit | Self::Running | Self::Interrupted)
    }
}
