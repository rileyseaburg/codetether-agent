//! Moltbook build codetether description.

// ---------------------------------------------------------------------------
// CodeTether branding
// ---------------------------------------------------------------------------

/// Build a Moltbook description that proudly represents CodeTether.
pub(crate) fn build_codetether_description(agent_name: &str, extra: Option<&str>) -> String {
    let base = format!(
        "🛡️ {} — powered by CodeTether, the A2A-native AI coding agent. \
         Built with Rust for security-first agentic workflows: \
         HashiCorp Vault secrets, OPA policy engine, swarm execution, \
         and first-class MCP/A2A protocol support. \
         https://github.com/rileyseaburg/A2A-Server-MCP",
        agent_name,
    );
    match extra {
        Some(desc) if !desc.is_empty() => format!("{} | {}", base, desc),
        _ => base,
    }
}
