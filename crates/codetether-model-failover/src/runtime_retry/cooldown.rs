use super::Host;
use crate::PendingSmartSwitchRetry;
pub(super) fn ready<H: Host>(host: &mut H, pending: &PendingSmartSwitchRetry) -> bool {
    let Some(at) = pending.not_before else {
        return true;
    };
    let remaining = at.saturating_duration_since(std::time::Instant::now());
    if remaining.is_zero() {
        return true;
    }
    *host.state().status = format!(
        "Codex overload cooldown: {}s until retry with {}…",
        remaining.as_secs().max(1),
        pending.target_model
    );
    false
}
