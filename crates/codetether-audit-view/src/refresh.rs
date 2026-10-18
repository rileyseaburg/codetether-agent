//! Refresh the cached view from the shared audit log.
use crate::AuditViewState;
use codetether_audit::try_audit_log;

/// Maximum number of entries the view caches / renders per category.
const SNAPSHOT_LIMIT: usize = 500;

/// Pull fresh entries from the global audit log into `state`.
///
/// Called on each TUI tick. When the global audit log has not been
/// initialized (e.g. running without the server), this is a cheap no-op.
///
/// # Arguments
/// * `state` - Cached view to update from the optional process-wide log.
/// # Returns
/// Unit after updating the snapshot, or immediately if no log is initialized.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let mut state = codetether_audit_view::AuditViewState::default();
/// codetether_audit_view::refresh_audit_snapshot(&mut state).await;
/// # }
/// ```
pub async fn refresh_audit_snapshot(state: &mut AuditViewState) {
    let Some(log) = try_audit_log() else {
        return;
    };
    let recent = log.recent(SNAPSHOT_LIMIT).await;
    state.entries = recent
        .into_iter()
        .filter(|e| state.filter.matches(e))
        .collect();
    if state.selected >= state.entries.len() {
        state.selected = state.entries.len().saturating_sub(1);
    }
    state.refresh_counter = state.refresh_counter.wrapping_add(1);
}
