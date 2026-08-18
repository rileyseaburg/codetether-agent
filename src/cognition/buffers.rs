//! Bounded ring buffers for events and snapshots.

use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};

use super::{MemorySnapshot, ThoughtEvent};

/// Append `event`, trim to `max_events`, and broadcast to subscribers.
pub(super) async fn push_event_internal(
    events: &Arc<RwLock<VecDeque<ThoughtEvent>>>,
    max_events: usize,
    event_tx: &broadcast::Sender<ThoughtEvent>,
    event: ThoughtEvent,
) {
    {
        let mut lock = events.write().await;
        lock.push_back(event.clone());
        while lock.len() > max_events {
            lock.pop_front();
        }
    }
    let _ = event_tx.send(event);
}

/// Append `snapshot` and trim to `max_snapshots`.
pub(super) async fn push_snapshot_internal(
    snapshots: &Arc<RwLock<VecDeque<MemorySnapshot>>>,
    max_snapshots: usize,
    snapshot: MemorySnapshot,
) {
    let mut lock = snapshots.write().await;
    lock.push_back(snapshot);
    while lock.len() > max_snapshots {
        lock.pop_front();
    }
}
