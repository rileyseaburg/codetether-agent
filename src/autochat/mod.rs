//! Shared autochat relay helpers used by TUI and CLI flows.

pub const AUTOCHAT_MAX_AGENTS: usize = 8;
pub const AUTOCHAT_DEFAULT_AGENTS: usize = 3;
pub const AUTOCHAT_MAX_ROUNDS: usize = 3;
pub const AUTOCHAT_MAX_DYNAMIC_SPAWNS: usize = 3;
pub const AUTOCHAT_SPAWN_CHECK_MIN_CHARS: usize = 800;
pub const AUTOCHAT_RLM_THRESHOLD_CHARS: usize = 6_000;
pub const AUTOCHAT_RLM_FALLBACK_CHARS: usize = 3_500;
pub const AUTOCHAT_STATUS_MAX_ROUNDS_REACHED: &str = "max_rounds_reached";
pub const AUTOCHAT_RLM_HANDOFF_QUERY: &str = "Prepare a concise relay handoff for the next specialist.\n\
Return FINAL(JSON) with this exact shape:\n\
{\"kind\":\"semantic\",\"file\":\"relay_handoff\",\"answer\":\"...\"}\n\
The \"answer\" must include:\n\
1) key conclusions,\n\
2) unresolved risks,\n\
3) one exact next action.\n\
Keep it concise and actionable.";
pub const AUTOCHAT_QUICK_DEMO_TASK: &str = "Self-organize into the right specialties for this task, then relay one concrete implementation plan with clear next handoffs.";

const REQUIRED_RELAY_CAPABILITIES: [&str; 4] =
    ["relay", "context-handoff", "rlm-aware", "autochat"];

pub fn ensure_required_relay_capabilities(capabilities: &mut Vec<String>) {
    for required in REQUIRED_RELAY_CAPABILITIES {
        if !capabilities.iter().any(|cap| cap == required) {
            capabilities.push(required.to_string());
        }
    }
}

pub fn parse_autochat_args<'a>(
    rest: &'a str,
    default_agents: usize,
    quick_demo_task: &'a str,
) -> Option<(usize, &'a str)> {
    let rest = rest.trim();
    if rest.is_empty() {
        return None;
    }

    let mut parts = rest.splitn(2, char::is_whitespace);
    let first = parts.next().unwrap_or("").trim();
    if first.is_empty() {
        return None;
    }

    if let Ok(count) = first.parse::<usize>() {
        let task = parts.next().unwrap_or("").trim();
        if task.is_empty() {
            Some((count, quick_demo_task))
        } else {
            Some((count, task))
        }
    } else {
        Some((default_agents, rest))
    }
}

pub fn normalize_for_convergence(text: &str, max_chars: usize) -> String {
    let mut normalized = String::with_capacity(text.len().min(512));
    let mut last_was_space = false;

    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() {
            normalized.push(ch.to_ascii_lowercase());
            last_was_space = false;
        } else if ch.is_whitespace() && !last_was_space {
            normalized.push(' ');
            last_was_space = true;
        }

        if normalized.len() >= max_chars {
            break;
        }
    }

    normalized.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::{
        AUTOCHAT_DEFAULT_AGENTS, AUTOCHAT_QUICK_DEMO_TASK, ensure_required_relay_capabilities,
        normalize_for_convergence, parse_autochat_args,
    };

    #[test]
    fn parse_autochat_args_with_default_count() {
        let parsed = parse_autochat_args(
            "implement relay checkpointing",
            AUTOCHAT_DEFAULT_AGENTS,
            AUTOCHAT_QUICK_DEMO_TASK,
        );
        assert_eq!(
            parsed,
            Some((AUTOCHAT_DEFAULT_AGENTS, "implement relay checkpointing"))
        );
    }

    #[test]
    fn parse_autochat_args_with_count_only_uses_demo_task() {
        let parsed = parse_autochat_args("4", AUTOCHAT_DEFAULT_AGENTS, AUTOCHAT_QUICK_DEMO_TASK);
        assert_eq!(parsed, Some((4, AUTOCHAT_QUICK_DEMO_TASK)));
    }

    #[test]
    fn normalize_for_convergence_ignores_case_and_symbols() {
        let a = normalize_for_convergence("Done! Next Step: Add tests.", 280);
        let b = normalize_for_convergence("done next step add tests", 280);
        assert_eq!(a, b);
    }

    #[test]
    fn ensure_required_relay_capabilities_adds_missing_caps() {
        let mut caps = vec!["planning".to_string(), "relay".to_string()];
        ensure_required_relay_capabilities(&mut caps);
        assert!(caps.iter().any(|cap| cap == "context-handoff"));
        assert!(caps.iter().any(|cap| cap == "rlm-aware"));
        assert!(caps.iter().any(|cap| cap == "autochat"));
    }
}
