//! Kubernetes pod health scoring for collapse decisions.

use crate::k8s::SubagentPodState;

pub(super) fn compute(state: Option<&SubagentPodState>) -> (f32, u32) {
    let Some(state) = state else { return (0.2, 1) };
    let reason = state
        .reason
        .as_deref()
        .unwrap_or_default()
        .to_ascii_lowercase();
    let phase = state.phase.to_ascii_lowercase();
    if reason.contains("oomkilled")
        || reason.contains("imagepullbackoff")
        || reason.contains("errimagepull")
    {
        return (0.0, 3);
    }
    if reason.contains("crashloopbackoff") || phase == "failed" {
        return (0.1, 2);
    }
    let mut score = 1.0f32;
    let mut signals = 0;
    if !state.ready {
        score -= 0.2;
    }
    if !reason.is_empty() {
        score -= 0.3;
        signals += 1;
    }
    if state.restart_count > 0 {
        score -= (state.restart_count.min(3) as f32) * 0.2;
        signals += 1;
    }
    (score.clamp(0.0, 1.0), signals)
}
