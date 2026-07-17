//! Credential, provenance, and identity environment for one command.

use serde_json::Value;
use std::path::Path;

#[path = "environment/context.rs"]
mod context;

pub(super) struct Environment {
    pub variables: Vec<(String, String)>,
    pub redactions: Vec<String>,
}

pub(super) async fn resolve(command: &str, cwd: &Path, args: &Value) -> Environment {
    let mut variables = crate::tool::bash_identity::git_identity_env_from_tool_args(args)
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect::<Vec<_>>();
    context::extend(&mut variables, args);
    let mut redactions = Vec::new();
    match crate::tool::bash_github::load_github_command_auth(command, cwd.to_str()).await {
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
            tracing::warn!(%error, "Failed to load GitHub auth for exec command");
        }
    }
    Environment {
        variables,
        redactions,
    }
}
