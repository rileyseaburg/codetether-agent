use super::Host;
use crate::should_execute_smart_switch;
use std::time::Instant;
/// Execute a scheduled retry or temporary-model restoration when ready.
/// Registry absence skips dispatch while retaining the original state transitions.
pub async fn execute<H: Host>(host: &mut H, registry: Option<&H::Provider>) {
    let Some(pending) = host.state().retry.pending.take() else {
        return;
    };
    if !super::cooldown::ready(host, &pending) {
        host.state().retry.pending = Some(pending);
        return;
    }
    if super::restore::restore(host, &pending).await {
        return;
    }
    let Some(current_model) = host.current_model() else {
        return;
    };
    if !should_execute_smart_switch(current_model, Some(&pending)) {
        return;
    }

    host.set_model(pending.target_model.clone()).await;

    *host.state().processing = true;
    host.begin_timing();
    *host.state().inflight = Some(pending.prompt.clone());
    *host.state().last_event = Some(Instant::now());
    *host.state().status = format!("Retrying with {}…", pending.target_model);

    if let Some(registry) = registry {
        host.submit(&pending.prompt, registry).await;
    }
    if let Some(restore) = crate::temporary_fallback::after_fallback(&pending) {
        host.state().retry.pending = Some(restore);
    }
}
