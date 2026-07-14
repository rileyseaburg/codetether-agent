//! System-message construction for spawned agents.

use std::path::PathBuf;

/// Build the workspace-aware system message for a spawned agent.
pub(super) fn build(name: &str, instructions: &str, workspace: Option<PathBuf>) -> String {
    let cwd = workspace
        .or_else(|| std::env::current_dir().ok())
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| "<unknown>".to_string());
    let contract = super::super::message_result::deliverable_contract();
    format!(
        "You are @{name}, a specialized sub-agent. {instructions}\n\n\
         Workspace cwd: {cwd}\n\
         All file paths you read/write should be relative to this cwd unless absolute.\n\
         Start with paths named in the task; use targeted search when a required path is unknown.\n\
         Do not stop after a plan: inspect, complete the deliverable, run focused verification, \
         and report concrete evidence or an explicit blocker. Avoid repeated discovery and narrate minimally.\n\n\
         {contract}"
    )
}
