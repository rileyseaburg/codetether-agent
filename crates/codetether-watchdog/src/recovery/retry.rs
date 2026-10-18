use super::{Host, Ui, give_up};
use crate::WATCHDOG_MAX_RESTARTS;
/// Retry the exact in-flight prompt after cancellation returns its session.
/// Returns without consuming input if no registry, prompt or session exists.
pub async fn execute<H: Host>(host: &mut H, registry: Option<&H::Provider>) {
    if host.ui().state().notification.is_none() || *host.ui().state().processing {
        return;
    }
    if *host.ui().state().restarts >= WATCHDOG_MAX_RESTARTS {
        return give_up(host.ui());
    }
    let Some(registry) = registry else {
        return;
    };
    let Some(prompt) = host.ui().state().inflight.clone() else {
        return;
    };
    let Some(session) = host.take_session() else {
        return;
    };
    *host.ui().state().inflight = Some(prompt.clone());
    *host.ui().state().notification = None;
    *host.ui().state().processing = true;
    host.ui().begin_timing();
    *host.ui().state().last_event = Some(std::time::Instant::now());
    let request = H::request(session, prompt, registry);
    if let Err(request) = host.submit(request).await {
        host.restore(request);
    }
}
