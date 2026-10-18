//! Runnable snapshots retain configuration without claiming or altering tasks.

use super::task;
use crate::KnativeTaskQueue;

mod pending_snapshot_preserves_configuration_and_queue_state;

const STATUSES: [&str; 8] = [
    "pending",
    "queued",
    "processing",
    "working",
    "completed",
    "failed",
    "cancelled",
    "unknown",
];
