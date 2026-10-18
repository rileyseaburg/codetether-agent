use super::Host;

/// Drain background sources in UI delivery order.
///
/// # Arguments
/// * `host` - Native state, event queues and notice handlers.
/// # Returns
/// Completes after the current event batch and all ready notices are applied.
/// # Examples
/// ```text
/// background::drain(&mut native_host).await;
/// ```
pub async fn drain<H: Host>(host: &mut H) {
    host.models();
    host.bus();
    host.queue_workers();
    host.display_worker();
    host.events().await;
    while let Some(notice) = host.next_notice() {
        apply_notice(host, notice).await;
    }
}

/// Apply a notice after draining the session events that precede it.
///
/// # Arguments
/// * `host` - Native event source and notice handler.
/// * `notice` - Completion notice already removed from its receiver.
/// # Returns
/// Completes after the preceding events and then the notice are applied.
/// # Examples
/// ```text
/// background::apply_notice(&mut native_host, notice).await;
/// ```
pub async fn apply_notice<H: Host>(host: &mut H, notice: H::Notice) {
    host.before_notice().await;
    host.notice(notice).await;
}
