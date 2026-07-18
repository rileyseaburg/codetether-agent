//! Atomic resident-order and pending-slot state.

use std::collections::VecDeque;
use std::sync::{Mutex, MutexGuard};

#[path = "state/reservation.rs"]
mod reservation;
#[path = "state/residents.rs"]
mod residents;

/// Synchronized resident ordering and pending-capacity accounting.
#[derive(Default)]
pub(super) struct ResidencyState {
    inner: Mutex<Inner>,
}

#[derive(Default)]
struct Inner {
    residents: VecDeque<String>,
    pending: usize,
}

impl ResidencyState {
    fn lock(&self) -> MutexGuard<'_, Inner> {
        self.inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}
