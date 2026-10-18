//! Latest-value pending slot with generation tracking.
mod defaults;
mod lifecycle;
/// Latest-value coalescing state independent of provider/runtime details.

/// # Examples
/// ```
/// let mut slot = codetether_proactive_planning::Slot::default();
/// assert!(slot.enqueue(|generation| generation));
/// assert_eq!(slot.take(), Some(1));
/// assert!(slot.settle());
/// ```
pub struct Slot<T> {
    pending: Option<T>,
    generation: u64,
    running: bool,
}
