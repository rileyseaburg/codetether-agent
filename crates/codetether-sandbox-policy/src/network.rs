//! Detect explicitly networked commands before sandbox spawning.
use crate::SandboxPolicy;
use anyhow::{Result, anyhow};
use std::path::Path;

const NETWORK_TOOLS: &[&str] = &[
    "curl", "wget", "nc", "netcat", "ssh", "scp", "sftp", "rsync", "telnet", "ftp",
];

/// Check whether a command requests networking forbidden by its policy.
///
/// # Arguments
/// * `policy` - Invocation permissions.
/// * `command` - Executable name or path.
/// * `args` - Command arguments, including any shell text.
/// # Returns
/// Advisory fallback markers when OS-level network isolation is still needed.
/// # Errors
/// Returns an error for recognized network commands when networking is denied.
/// # Examples
/// ```rust
/// let policy = codetether_sandbox_policy::SandboxPolicy::default();
/// assert!(codetether_sandbox_policy::validate_network(&policy, "curl", &[]).is_err());
/// ```
pub fn validate(policy: &SandboxPolicy, command: &str, args: &[String]) -> Result<Vec<String>> {
    if policy.allow_network {
        return Ok(Vec::new());
    }
    if mentions_network_tool(command, args) {
        return Err(anyhow!(
            "Sandbox denies network access; enable Network access in Settings, then retry"
        ));
    }
    Ok(vec!["network_marker_only".to_string()])
}

fn mentions_network_tool(command: &str, args: &[String]) -> bool {
    is_network_command(command) || args.iter().any(|arg| mentions_shell_word(arg))
}

fn is_network_command(command: &str) -> bool {
    let command = command.to_ascii_lowercase();
    let name = Path::new(&command)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(command.as_str());
    NETWORK_TOOLS.contains(&name)
}

fn mentions_shell_word(arg: &str) -> bool {
    let arg = arg.to_ascii_lowercase();
    let dequoted = arg.replace(['"', '\'', '\\'], "");
    NETWORK_TOOLS
        .iter()
        .any(|tool| contains_shell_word(&arg, tool) || contains_shell_word(&dequoted, tool))
}

fn contains_shell_word(haystack: &str, needle: &str) -> bool {
    haystack
        .split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '-' || ch == '_'))
        .any(|word| word == needle)
}
