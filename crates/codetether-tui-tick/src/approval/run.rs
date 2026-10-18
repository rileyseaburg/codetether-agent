use super::Host;
/// Reconcile stale approval overlays and report whether the display changed.
/// # Arguments
/// Host supplies the native queue and mutable overlay fields.
/// # Returns
/// False when no stale approvals were removed, otherwise true after updates.
/// # Examples
/// ```ignore
/// let changed = reconcile(&mut ui);
/// ```
pub fn reconcile<H: Host>(host: &mut H) -> bool {
    let removed = host.remove_stale();
    if removed == 0 {
        return false;
    }
    let remaining = host.remaining();
    *host.state().waiting = remaining > 0;
    *host.state().preview_scroll = 0;
    *host.state().status = if remaining == 0 {
        format!("Reconciled {removed} externally resolved approval request(s)")
    } else {
        format!("Reconciled {removed} externally resolved approval request(s); {remaining} pending")
    };
    true
}
