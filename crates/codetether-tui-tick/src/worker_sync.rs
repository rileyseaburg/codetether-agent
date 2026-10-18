//! Cursor-gated synchronization before drawing the worker roster.
/// Whether the current bus cursor differs from the last synchronized cursor.
/// # Examples
/// ```
/// assert!(codetether_tui_tick::worker_sync::should_sync(None, 0));
/// assert!(!codetether_tui_tick::worker_sync::should_sync(Some(7), 7));
/// ```
pub fn should_sync(synced: Option<u64>, current: u64) -> bool {
    synced != Some(current)
}
/// Synchronize once per changed bus cursor, recording the cursor first.
/// # Arguments
/// Synced holds the previous cursor; current is the live bus cursor; sync
/// performs the existing native roster synchronization.
/// # Returns
/// No value; leaves state untouched when the cursor is already synchronized.
/// # Examples
/// ```
/// let mut cursor = None;
/// codetether_tui_tick::worker_sync::before_draw(&mut cursor, 1, || {});
/// assert_eq!(cursor, Some(1));
/// ```
pub fn before_draw(synced: &mut Option<u64>, current: u64, sync: impl FnOnce()) {
    if !should_sync(*synced, current) {
        return;
    }
    *synced = Some(current);
    sync();
}
#[cfg(test)]
#[path = "worker_sync_tests.rs"]
mod tests;
