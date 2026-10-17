//! Bounded ring buffers for events and snapshots.

use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};

use crate::{MemorySnapshot, ThoughtEvent};

/// Append `event`, trim to `max_events`, and broadcast to subscribers.
///
/// # Arguments
/// * `events` - Destination for generated events, or shared bounded event history.
/// * `max_events` - Maximum retained event count.
/// * `event_tx` - Broadcast channel for events after storage is updated.
/// * `event` - Event to append and broadcast.
///
/// # Returns
/// Updates the supplied state; no separate value is returned.
///
/// # Examples
/// ```no_run
/// # use codetether_cognition_core::*;
/// # use codetether_cognition_core::beliefs::Belief;
/// # use chrono::{DateTime, Utc};
/// # use std::{collections::{HashMap, VecDeque}, sync::Arc};
/// # use tokio::sync::{RwLock, broadcast};
/// # async fn example(events: &Arc<RwLock<VecDeque<ThoughtEvent>>>, max_events: usize, event_tx: &broadcast::Sender<ThoughtEvent>, event: ThoughtEvent,) {
/// let _result = codetether_cognition_core::thought::buffers::push_event_internal(events, max_events, event_tx, event).await;
/// # }
/// ```
pub async fn push_event_internal(
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
///
/// # Arguments
/// * `snapshots` - Shared snapshot history.
/// * `max_snapshots` - Maximum retained snapshot count.
/// * `snapshot` - Snapshot to append.
///
/// # Returns
/// Updates the supplied state; no separate value is returned.
///
/// # Examples
/// ```no_run
/// # use codetether_cognition_core::*;
/// # use codetether_cognition_core::beliefs::Belief;
/// # use chrono::{DateTime, Utc};
/// # use std::{collections::{HashMap, VecDeque}, sync::Arc};
/// # use tokio::sync::{RwLock, broadcast};
/// # async fn example(snapshots: &Arc<RwLock<VecDeque<MemorySnapshot>>>, max_snapshots: usize, snapshot: MemorySnapshot,) {
/// let _result = codetether_cognition_core::thought::buffers::push_snapshot_internal(snapshots, max_snapshots, snapshot).await;
/// # }
/// ```
pub async fn push_snapshot_internal(
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
