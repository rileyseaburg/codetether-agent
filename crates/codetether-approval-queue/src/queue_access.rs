//! FIFO queue access and mutation operations.

use codetether_approval_core::interaction::LiveApprovalRequest;

use super::{ApprovalSnapshot, queue};

/// Insert a request at the back, replacing any queued copy of its ID.
/// # Arguments
/// * `request` - Live approval request to display.
/// # Returns
/// The inserted snapshot.
/// # Panics
/// Panics if the process-wide queue mutex was poisoned.
/// # Examples
/// ```no_run
/// let request = codetether_approval_core::interaction::LiveApprovalRequest::new("id".into(), "call".into(), "write".into(), "write".into(), "note".into(), "save".into());
/// let _ = codetether_approval_queue::push(request);
/// ```
pub fn push(request: LiveApprovalRequest) -> ApprovalSnapshot {
    let snapshot = ApprovalSnapshot::from(request);
    let mut guard = queue().lock().expect("approval queue lock");
    guard.retain(|item| item.id != snapshot.id);
    guard.push_back(snapshot.clone());
    snapshot
}

/// Clone the currently displayed request, if any.
/// # Arguments
/// * Uses the process-wide display queue.
/// # Returns
/// A cloned head snapshot or None.
/// # Panics
/// Panics if the process-wide queue mutex was poisoned.
/// # Examples
/// ```no_run
/// let _ = codetether_approval_queue::active();
/// ```
pub fn active() -> Option<ApprovalSnapshot> {
    queue()
        .lock()
        .expect("approval queue lock")
        .front()
        .cloned()
}

/// Return the ID of the currently displayed request.
/// # Arguments
/// * Uses the process-wide display queue.
/// # Returns
/// The head request ID or None.
/// # Panics
/// Panics if the process-wide queue mutex was poisoned.
/// # Examples
/// ```no_run
/// let _ = codetether_approval_queue::active_id();
/// ```
pub fn active_id() -> Option<String> {
    active().map(|item| item.id)
}

/// Return the pending queue length.
/// # Arguments
/// * Uses the process-wide display queue.
/// # Returns
/// The number of queued display snapshots.
/// # Panics
/// Panics if the process-wide queue mutex was poisoned.
/// # Examples
/// ```no_run
/// let _ = codetether_approval_queue::len();
/// ```
pub fn len() -> usize {
    queue().lock().expect("approval queue lock").len()
}

/// Remove a request by ID without issuing an approval decision.
/// # Arguments
/// * `id` - Request ID to remove.
/// # Returns
/// Removes all matching snapshots.
/// # Panics
/// Panics if the process-wide queue mutex was poisoned.
/// # Examples
/// ```no_run
/// codetether_approval_queue::resolve("id");
/// ```
pub fn resolve(id: &str) {
    queue()
        .lock()
        .expect("approval queue lock")
        .retain(|item| item.id != id);
}

/// Clear display state without issuing approval decisions; useful for isolated clients.
/// # Arguments
/// * Uses the process-wide display queue.
/// # Returns
/// Clears all snapshots.
/// # Panics
/// Panics if the process-wide queue mutex was poisoned.
/// # Examples
/// ```no_run
/// codetether_approval_queue::reset();
/// ```
pub fn reset() {
    queue().lock().expect("approval queue lock").clear();
}
