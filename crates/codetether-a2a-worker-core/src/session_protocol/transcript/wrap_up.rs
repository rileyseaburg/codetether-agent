use super::Host;

/// Append a budget wrap-up prompt when the step reaches the shared boundary.
/// # Arguments
/// `host` owns the transcript; `step` and `max_steps` determine the boundary.
/// # Returns
/// Whether a wrap-up message was appended.
/// # Examples
/// ```no_run
/// # fn wrap_up(host: &mut impl codetether_a2a_worker_core::session_protocol::transcript::Host) {
/// let _inserted = codetether_a2a_worker_core::session_protocol::transcript::maybe_inject(host, 9, 10);
/// # }
/// ```
pub fn maybe_inject(host: &mut impl Host, step: usize, max_steps: usize) -> bool {
    if let Some(message) = crate::session_protocol::wrap_up_message(step, max_steps) {
        host.append(message);
        true
    } else {
        false
    }
}
