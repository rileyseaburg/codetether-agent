//! Initialize an idle empty generation slot.
use super::Slot;

impl<T> Default for Slot<T> {
    fn default() -> Self {
        Self {
            pending: None,
            generation: 0,
            running: false,
        }
    }
}
