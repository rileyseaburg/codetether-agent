//! Evidence labels recorded for every final key-result outcome.
use super::Report;
pub(super) fn evidence(report: &Report<'_>) -> Vec<String> {
    vec![
        format!("relay:{}", report.relay_id),
        format!("turns:{}", report.turns),
        format!("agents:{}", report.agent_count),
        format!("status:{}", report.status),
        format!("rlm_handoffs:{}", report.rlm_handoff_count),
        format!("dynamic_spawns:{}", report.dynamic_spawn_count),
    ]
}
