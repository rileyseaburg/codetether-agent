//! Peer startup naming.

/// Auto-derive a stable, human-readable agent name from
/// `<short-host>-<repo-basename>-<short-pid>`. Falls back gracefully if any
/// component is unavailable.
///
/// # Arguments
/// Takes no arguments; reads the hostname, working directory and process ID.
/// # Returns
/// A lowercase peer name with non-alphanumeric characters replaced by dashes.
/// # Examples
/// ```rust
/// use codetether_a2a_discovery::naming::auto_agent_name;
/// assert!(!auto_agent_name().is_empty());
/// ```
pub fn auto_agent_name() -> String {
    let host_full = gethostname::gethostname()
        .into_string()
        .unwrap_or_else(|_| "host".to_string());
    let host_short = host_full
        .split('.')
        .next()
        .unwrap_or(&host_full)
        .to_string();
    let repo = std::env::current_dir()
        .ok()
        .as_ref()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .unwrap_or("repo")
        .to_string();
    let pid = std::process::id();
    let short_pid = format!("{:04x}", pid & 0xffff);
    let raw = format!("{host_short}-{repo}-{short_pid}");
    sanitize_name(&raw)
}

fn sanitize_name(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect()
}
