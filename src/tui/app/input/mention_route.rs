//! Detect and route @mention messages to spawned sub-agents.
//!
//! Routing wiring for per-agent channels is not yet in place; for now
//! the parser is invoked at submit time and the prompt is decorated
//! with a `[to @<name>]` prefix so the active model still sees the
//! intended target.

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

/// Extract an @mention target from input text.
/// Returns `Some((agent_name, message))` if the input starts with `@`
/// followed by a non-empty name, a space, and a non-empty message.
pub fn parse_mention(input: &str) -> Option<(String, String)> {
    let trimmed = input.trim();
    if !trimmed.starts_with('@') {
        return None;
    }
    let rest = &trimmed[1..];
    let space_pos = rest.find(' ')?;
    let name = rest[..space_pos].to_string();
    let msg = rest[space_pos..].trim().to_string();
    if name.is_empty() || msg.is_empty() {
        return None;
    }
    Some((name, msg))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_mention() {
        let (name, msg) = parse_mention("@forge fix the tests").unwrap();
        assert_eq!(name, "forge");
        assert_eq!(msg, "fix the tests");
    }

    #[test]
    fn no_mention() {
        assert!(parse_mention("regular message").is_none());
    }

    #[test]
    fn empty_message() {
        assert!(parse_mention("@forge").is_none());
    }
}
