use crate::provider::ToolDefinition;
use anyhow::Result;

pub fn provider_has_flaky_native_tool_calling(provider_name: &str, model: &str) -> bool {
    let provider = provider_name.to_ascii_lowercase();
    let model = model.to_ascii_lowercase();
    provider == "minimax"
        || provider == "minimax-credits"
        || model.contains("minimax")
        || model.contains("m2.5")
}

pub fn assistant_claims_imminent_tool_use(text: &str, tool_definitions: &[ToolDefinition]) -> bool {
    let lower = text.trim().to_ascii_lowercase();
    if lower.is_empty() {
        return false;
    }

    let explicit_markers = [
        "tool call",
        "tool calls",
        "use a tool",
        "use tools",
        "call the",
        "invoke the",
        "let me use",
        "i'll use",
        "i will use",
        "i'm going to use",
        "i am going to use",
    ];
    if explicit_markers.iter().any(|m| lower.contains(m)) {
        return true;
    }

    let action_markers = [
        "let me check",
        "let me inspect",
        "let me search",
        "let me read",
        "let me open",
        "first, i'll",
        "first i will",
        "i'll check",
        "i'll inspect",
        "i'll search",
        "i'll read",
        "i'll open",
        "i'll look through",
        "i'll examine",
        "i will check",
        "i will inspect",
        "i will search",
        "i will read",
        "i will open",
        "i will examine",
    ];
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

pub fn should_retry_missing_native_tool_call(
    provider_name: &str,
    model: &str,
    retry_count: u8,
    tool_definitions: &[ToolDefinition],
    assistant_text: &str,
    has_tool_calls: bool,
    native_tool_promise_retry_max_retries: u8,
) -> bool {
    if retry_count >= native_tool_promise_retry_max_retries
        || has_tool_calls
        || tool_definitions.is_empty()
        || !provider_has_flaky_native_tool_calling(provider_name, model)
    {
        return false;
    }

    assistant_claims_imminent_tool_use(assistant_text, tool_definitions)
}

pub fn choose_default_provider<'a>(providers: &'a [&'a str]) -> Option<&'a str> {
    let preferred = [
        "openai",
        "anthropic",
        "github-copilot",
        "openai-codex",
        "zai",
        "minimax",
        "openrouter",
        "novita",
        "moonshotai",
        "google",
    ];
    for name in preferred {
        if let Some(found) = providers.iter().copied().find(|p| *p == name) {
            return Some(found);
        }
    }
    providers.first().copied()
}

pub fn resolve_provider_for_session_request<'a>(
    providers: &'a [&'a str],
    explicit_provider: Option<&str>,
) -> Result<&'a str> {
    if let Some(explicit) = explicit_provider {
        if let Some(found) = providers.iter().copied().find(|p| *p == explicit) {
            return Ok(found);
        }
        anyhow::bail!(
            "Provider '{}' selected explicitly but is unavailable. Available providers: {}",
            explicit,
            providers.join(", ")
        );
    }

    choose_default_provider(providers).ok_or_else(|| anyhow::anyhow!("No providers available"))
}

pub fn prefers_temperature_one(model: &str) -> bool {
    let normalized = model.to_ascii_lowercase();
    normalized.contains("kimi-k2") || normalized.contains("glm-") || normalized.contains("minimax")
}

/// Returns true for models where the `temperature` parameter is deprecated
/// and must not be sent (causes a 400 Bad Request error).
/// Claude Opus 4.7 removed temperature support in favor of adaptive reasoning.
pub fn temperature_is_deprecated(model: &str) -> bool {
    let normalized = model.to_ascii_lowercase();
    normalized.contains("opus-4-7")
        || normalized.contains("opus-4.7")
        || normalized.contains("4.7-opus")
        || normalized.contains("4-7-opus")
        || normalized.contains("opus_4_7")
        || normalized.contains("opus_47")
}

#[cfg(test)]
mod tests {
    use super::temperature_is_deprecated;

    #[test]
    fn detects_opus_47_aliases() {
        assert!(temperature_is_deprecated("claude-opus-4-7"));
        assert!(temperature_is_deprecated("claude-opus-4.7"));
        assert!(temperature_is_deprecated("claude-4.7-opus"));
        assert!(temperature_is_deprecated("claude-4-7-opus"));
        assert!(temperature_is_deprecated("claude-opus_4_7"));
        assert!(temperature_is_deprecated("claude-opus_47"));
        assert!(temperature_is_deprecated("us.anthropic.claude-opus-4-7"));
    }

    #[test]
    fn non_deprecated_models_return_false() {
        assert!(!temperature_is_deprecated("claude-sonnet-4"));
        assert!(!temperature_is_deprecated("claude-opus-4-6"));
        assert!(!temperature_is_deprecated("gpt-4o"));
    }
}
