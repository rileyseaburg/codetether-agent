//! Approval and denial shortcuts target pending requests only.

use super::Host;
use codetether_approval_queue as approval_queue;

/// Run a decision command when an approval is pending.
/// # Arguments
/// * `host` — Frontend state and command bindings.
/// * `command` — Approval command to invoke.
/// # Returns
/// Whether the action was consumed.
/// # Examples
/// ```no_run
/// # fn example(host: &mut impl codetether_approval_view::interactive::Host) {
/// let _ = codetether_approval_view::interactive::decide(host, "/approve");
/// # }
/// ```
pub fn decide(host: &mut impl Host, command: &str) -> bool {
    if !pending() {
        return false;
    }
    host.run_command(command)
}

fn pending() -> bool {
    approval_queue::active_id()
        .or_else(codetether_approval_core::interaction::pending_query::latest_id)
        .is_some()
}
