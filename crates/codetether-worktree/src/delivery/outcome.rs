//! Typed result of one delivery attempt.

use serde::Serialize;
use std::path::PathBuf;

/// What happened to the target checkout during a delivery.
///
/// # Examples
///
/// ```
/// use codetether_worktree::DeliveryStatus;
/// for status in [DeliveryStatus::Applied, DeliveryStatus::Rejected] {
///     match status {
///         DeliveryStatus::Applied => assert!(status.is_delivered()),
///         DeliveryStatus::AlreadyPresent | DeliveryStatus::Empty => {}
///         DeliveryStatus::Rejected => assert!(!status.is_delivered()),
///     }
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryStatus {
    /// The patch was applied to the target working tree as uncommitted edits.
    Applied,
    /// The target already contains the patch; nothing was changed.
    AlreadyPresent,
    /// The branch has no net patch; nothing was changed.
    Empty,
    /// The patch does not apply to the target; nothing was changed.
    Rejected,
}

impl DeliveryStatus {
    /// Whether the target now contains the worker's changes.
    pub fn is_delivered(self) -> bool {
        !matches!(self, Self::Rejected)
    }

    /// Stable ledger spelling.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Applied => "applied",
            Self::AlreadyPresent => "already_present",
            Self::Empty => "empty",
            Self::Rejected => "rejected",
        }
    }
}

/// Recorded outcome of delivering one worker branch.
#[derive(Clone, Debug, Serialize)]
pub struct Delivery {
    /// SQLite ledger row identifier.
    pub id: i64,
    /// Outcome for the target checkout.
    pub status: DeliveryStatus,
    /// Delivered worker branch.
    pub branch: String,
    /// Branch tip commit that was delivered.
    pub head: String,
    /// Merge base the patch was computed from.
    pub base: String,
    /// Receiving checkout root.
    pub target: PathBuf,
    /// Repository-relative files in the patch.
    pub files: Vec<String>,
    /// Git diagnostic for rejected patches; empty otherwise.
    pub detail: String,
}

impl Delivery {
    /// One human-readable paragraph describing the outcome.
    pub fn summary(&self) -> String {
        let target = self.target.display();
        let files = self.files.len();
        match self.status {
            DeliveryStatus::Applied => format!(
                "Applied {files} file(s) from '{}' to {target} as uncommitted changes; \
                 review with `git diff` (no commits were made). Ledger delivery #{}.",
                self.branch, self.id
            ),
            DeliveryStatus::AlreadyPresent => format!(
                "Changes from '{}' are already present in {target}. Ledger delivery #{}.",
                self.branch, self.id
            ),
            DeliveryStatus::Empty => format!("Branch '{}' had no changes to deliver", self.branch),
            DeliveryStatus::Rejected => format!(
                "Changes from '{}' do not apply cleanly to {target}; it was left untouched and \
                 the worker branch was kept. Ledger delivery #{}. git: {}",
                self.branch,
                self.id,
                self.detail.trim()
            ),
        }
    }
}
