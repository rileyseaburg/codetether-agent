//! Shared metadata-keyed goal-state cache with native journal coercion.
pub fn load(log: &super::TaskLog) -> anyhow::Result<super::TaskState> {
    codetether_goal_runtime::state_cache::load(log)
}
