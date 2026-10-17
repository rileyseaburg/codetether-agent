//! Bounded in-memory history and cursor-based draining of bus envelopes.
//!
//! [BusRecorder] stores clones in publication order and recovers poisoned locks.
//! Consumers can query recent messages or drain the retained window after a cursor.
use crate::BusEnvelope;
use std::{
    collections::VecDeque,
    sync::{
        Mutex,
        atomic::{AtomicU64, Ordering},
    },
};
mod construct;
mod drain;
mod query;
mod record;

/// Bounded, thread-safe history of recent bus envelopes.
#[derive(Debug)]
///
/// # Examples
/// ```
/// let recorder = codetether_bus_core::recorder::BusRecorder::new(16);
/// assert_eq!(recorder.cursor(), 0);
/// assert!(recorder.recent(10, None).is_empty());
/// ```
pub struct BusRecorder {
    cap: usize,
    pub(super) buf: Mutex<VecDeque<BusEnvelope>>,
    /// Total envelopes ever recorded (monotonic; used for lossless cursors).
    pub(super) pushed: AtomicU64,
}
