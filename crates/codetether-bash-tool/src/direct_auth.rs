//! Add repository-scoped GitHub credentials to a direct command.
use super::Call;
use codetether_github_auth::{GitHubCommandAuth, load_github_command_auth};
use tokio::process::Command;
pub(super) async fn configure(cmd: &mut Command, call: &Call<'_>) -> Option<GitHubCommandAuth> {
    let github_auth = match load_github_command_auth(
        call.command,
        call.effective_cwd.as_deref().and_then(|dir| dir.to_str()),
    )
    .await
    {
        Ok(auth) => auth,
        Err(err) => {
            tracing::warn!(target: "codetether_agent::tool::bash", error = %err, "Failed to load GitHub auth for bash command");
            None
        }
    };
    if let Some(auth) = github_auth.as_ref() {
        for (key, value) in &auth.env {
            cmd.env(key, value);
        }
    }

    github_auth
}
