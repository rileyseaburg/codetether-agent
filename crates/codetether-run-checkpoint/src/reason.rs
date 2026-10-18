//! Reason a resumable run checkpoint was created.
use serde::{Deserialize, Serialize};

/// The reason a resumable run checkpoint was created.
/// # Examples
/// ```rust
/// use codetether_run_checkpoint::CheckpointReason;
/// match CheckpointReason::MaxStepsExhausted {
///     CheckpointReason::MaxStepsExhausted => {},
/// }
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointReason {
    /// The run stopped because its configured maximum step budget was reached.
    MaxStepsExhausted,
}
