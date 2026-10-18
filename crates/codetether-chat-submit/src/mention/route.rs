//! Detect and route @mention messages to spawned sub-agents.
//!
//! Routing wiring for per-agent channels is not yet in place; for now
//! the parser is invoked at submit time and the prompt is decorated
//! with a `[to @<name>]` prefix so the active model still sees the
//! intended target.

use super::parse_mention;

/// Decorate `input` with a `[to @<name>]` prefix when it targets a
/// spawned agent, either via an explicit `@mention` or the currently
/// focused agent. Commands (`/`, `!`) and empty input are passed through
/// unchanged so focus never hijacks slash dispatch.
pub fn route_prompt(input: &str, focused_agent: Option<&str>) -> String {
    if let Some((agent_name, msg)) = parse_mention(input) {
        tracing::info!(agent = %agent_name, "Mention detected, routing as prefixed message");
        return format!("[to @{}] {}", agent_name, msg);
    }
    match focused_agent {
        Some(name) if !input.is_empty() && !input.starts_with(['/', '!']) => {
            tracing::info!(agent = %name, "Focused agent active, routing prompt");
            format!("[to @{}] {}", name, input)
        }
        _ => input.to_string(),
    }
}
