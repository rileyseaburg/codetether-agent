use super::Host;
use crate::PendingSmartSwitchRetry;
pub(super) async fn restore<H: Host>(host: &mut H, pending: &PendingSmartSwitchRetry) -> bool {
    let Some(fallback) = pending.restore_from.as_deref() else {
        return false;
    };
    if *host.state().processing {
        *host.state().status =
            format!("Using fallback {fallback}; restore is waiting for completion…");
        host.state().retry.pending = Some(pending.clone());
        return true;
    }
    let Some(model) = host.current_model() else {
        return true;
    };
    if model != Some(fallback) {
        *host.state().status = "Automatic Codex restore skipped after a manual model change".into();
        return true;
    }
    let Some(original) = pending.restore_model.as_deref() else {
        return true;
    };
    host.set_model(original.to_string()).await;
    host.refresh_view();
    host.state().retry.clear_attempts();
    *host.state().status = format!("Codex overload cooldown ended; restored {original}");
    true
}
