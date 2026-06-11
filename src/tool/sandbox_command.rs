use super::sandbox_landlock;
use std::collections::HashMap;
use std::path::Path;

pub(super) fn build(
    program: &str,
    args: &[String],
    work_dir: &Path,
    env: &HashMap<String, String>,
    landlock: Option<sandbox_landlock::Rules>,
    max_memory_bytes: u64,
) -> (tokio::process::Command, Vec<String>) {
    let mut cmd = tokio::process::Command::new(program);
    cmd.args(args).current_dir(work_dir).env_clear().envs(env);
    super::super::bash_noninteractive::configure(&mut cmd);
    sandbox_landlock::apply(&mut cmd, landlock);
    let fallbacks = super::super::sandbox_limits::apply_memory_limit(&mut cmd, max_memory_bytes);
    (cmd, fallbacks)
}
