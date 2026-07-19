//! Results returned by the mux lease authority.

use super::WorktreeLease;
use serde::{Deserialize, Serialize};

/// Outcome of one coordination request.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub(crate) enum CoordinationReply {
    Acquired { leases: Vec<WorktreeLease> },
    Blocked { conflicts: Vec<WorktreeLease> },
    Renewed { count: usize },
    Released { count: usize },
    Snapshot { leases: Vec<WorktreeLease> },
}
