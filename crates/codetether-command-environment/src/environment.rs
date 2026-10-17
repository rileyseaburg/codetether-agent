//! Credential, provenance, and identity environment for one command.

use serde_json::Value;
use std::path::Path;

#[path = "environment/context.rs"]
mod context;

/// Prepared child variables and secret strings to redact from its output.
///
/// # Examples
/// ```rust
/// let environment = codetether_command_environment::Environment { variables: vec![], redactions: vec![] };
/// assert!(environment.variables.is_empty());
/// ```
pub struct Environment {
    /// Ordered variable assignments applied to the command.
    pub variables: Vec<(String, String)>,
    /// Credential values removed from model-facing command output.
    pub redactions: Vec<String>,
}

/// Prepare Git identity, runtime context and applicable GitHub credentials.
///
/// # Arguments
/// * `command` - Shell text used to determine whether GitHub auth applies.
/// * `cwd` - Repository context for scoped authentication.
/// * `args` - Tool arguments carrying identity and context fields.
/// # Returns
/// Ordered assignments and redactions; authentication errors are logged.
/// # Examples
/// ```rust,no_run
/// # async fn example() {
/// let _env = codetether_command_environment::resolve(
///     "git status", std::path::Path::new("."), &serde_json::json!({}),
/// ).await;
/// # }
/// ```
pub async fn resolve(command: &str, cwd: &Path, args: &Value) -> Environment {
    let mut variables = crate::git_identity_env_from_tool_args(args)
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect::<Vec<_>>();
    context::extend(&mut variables, args);
    let mut redactions = Vec::new();
    match codetether_github_auth::load_github_command_auth(command, cwd.to_str()).await {
        Ok(Some(auth)) => {
            variables.extend(
                auth.env
                    .into_iter()
                    .map(|(key, value)| (key.to_string(), value)),
            );
            redactions = auth.redactions;
        }
        Ok(None) => {}
        Err(error) => {
            tracing::warn!(target: "codetether_agent::tool::exec_command::environment",%error, "Failed to load GitHub auth for exec command");
        }
    }
    Environment {
        variables,
        redactions,
    }
}
