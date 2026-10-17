//! Prepare the shell process and working directory.
use super::Call;
use super::codetether_runtime_env;
use codetether_github_auth::GitHubCommandAuth;
use tokio::process::Command;
pub(super) async fn prepare(call: &Call<'_>) -> (Command, Option<GitHubCommandAuth>) {
    let shell = codetether_command_shell::resolve();
    let mut cmd = Command::new(&shell.program);
    cmd.kill_on_drop(true);
    cmd.args(&shell.prefix_args).arg(&call.wrapped_command);
    codetether_command_shell::configure(&mut cmd);
    if let Some((codetether_bin, path)) = codetether_runtime_env() {
        cmd.env("CODETETHER_BIN", codetether_bin).env("PATH", path);
    }

    super::direct_environment::configure(&mut cmd, call.args);
    let github_auth = super::direct_auth::configure(&mut cmd, call).await;
    if let Some(dir) = call.effective_cwd.as_deref() {
        cmd.current_dir(dir);
    }

    (cmd, github_auth)
}
