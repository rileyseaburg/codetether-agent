//! Wait-duration annotation for final lease acquisition outcomes.

use super::CoordinationReply;

impl CoordinationReply {
    pub(super) fn with_waited(self, waited_ms: u64) -> Self {
        match self {
            Self::Acquired { leases, .. } => Self::Acquired { leases, waited_ms },
            Self::Blocked {
                conflicts,
                retry_after_ms,
                ..
            } => Self::Blocked {
                conflicts,
                waited_ms,
                retry_after_ms,
            },
            Self::Renewed { count } => Self::Renewed { count },
            Self::Released { count } => Self::Released { count },
            Self::Snapshot { leases } => Self::Snapshot { leases },
        }
    }
}
