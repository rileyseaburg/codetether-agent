//! Recognize assistant prose promising an imminent registered tool call.
use codetether_provider_core::ToolDefinition;

/// Apply the established assistant claims imminent tool use policy.
/// # Arguments
/// Receives the provider, request or error inputs indicated by the signature.
/// # Returns
/// The policy decision without invoking providers or mutating session state.
/// # Examples
/// ```
/// assert!(codetether_provider_policy::assistant_claims_imminent_tool_use("I will use tools", &[]));
/// ```
pub fn assistant_claims_imminent_tool_use(text: &str, tool_definitions: &[ToolDefinition]) -> bool {
    let lower = text.trim().to_ascii_lowercase();
    if lower.is_empty() {
        return false;
    }

    let explicit_markers = super::promise_markers::EXPLICIT;
    if explicit_markers.iter().any(|m| lower.contains(m)) {
        return true;
    }

    let action_markers = super::promise_markers::ACTIONS;
    if action_markers.iter().any(|m| lower.contains(m))
        && tool_definitions.iter().any(|tool| {
            let name = tool.name.to_ascii_lowercase();
            lower.contains(&name)
                || matches!(
                    name.as_str(),
                    "bash"
                        | "read"
                        | "write"
                        | "edit"
                        | "advanced_edit"
                        | "multiedit"
                        | "codesearch"
                        | "glob"
                        | "grep"
                        | "ls"
                        | "cat"
                        | "lsp"
                )
        })
    {
        return true;
    }

    false
}
