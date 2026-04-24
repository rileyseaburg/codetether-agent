use crate::tool::sandbox::SandboxPolicy;
use anyhow::{Result, anyhow};

const NETWORK_TOOLS: &[&str] = &[
    "curl", "wget", "nc", "netcat", "ssh", "scp", "sftp", "rsync", "telnet", "ftp",
];

pub fn validate(policy: &SandboxPolicy, command: &str, args: &[String]) -> Result<Vec<String>> {
    if policy.allow_network {
        return Ok(Vec::new());
    }
    if mentions_network_tool(command, args) {
        return Err(anyhow!(
            "Sandbox policy denies network access for this command"
        ));
    }
    Ok(vec!["network_marker_only".to_string()])
}

fn mentions_network_tool(command: &str, args: &[String]) -> bool {
    let haystack = std::iter::once(command)
        .chain(args.iter().map(String::as_str))
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase();
    NETWORK_TOOLS
        .iter()
        .any(|tool| contains_shell_word(&haystack, tool))
}

fn contains_shell_word(haystack: &str, needle: &str) -> bool {
    haystack
        .split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '-' || ch == '_'))
        .any(|word| word == needle)
}
