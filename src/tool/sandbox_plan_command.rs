//! Build a child command from a resolved sandbox plan.

use super::{sandbox_command, sandbox_plan_state::PlanState};
use std::collections::HashMap;
use std::path::Path;

pub(super) fn build(
    state: &mut PlanState,
    work_dir: &Path,
    env: &HashMap<String, String>,
    max_memory_bytes: u64,
) -> (tokio::process::Command, Vec<String>) {
    sandbox_command::build(
        &state.program,
        &state.args,
        work_dir,
        env,
        state.landlock.take(),
        max_memory_bytes,
        state.seccomp.as_ref(),
        state.apply_seccomp,
    )
}