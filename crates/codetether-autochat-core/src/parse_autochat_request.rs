//! Shared auto-chat request policy.
use super::*;

/// Parse optional agent count and the no-PRD flag from a request.
///
/// # Arguments
/// * `rest` - Request text following the command name.
/// * `default_agents` - Count used when no number is supplied.
/// * `quick_demo_task` - Task used for a count-only request.
///
/// # Returns
/// Parsed request details, or `None` when no task or count remains.
///
/// # Examples
/// ```
/// use codetether_autochat_core::parse_autochat_request;
/// let request = parse_autochat_request("--no-prd 2 review", 3, "demo").unwrap();
/// assert_eq!(request.agent_count, 2);
/// assert!(request.bypass_prd);
/// ```
pub fn parse_autochat_request(
    rest: &str,
    default_agents: usize,
    quick_demo_task: &str,
) -> Option<ParsedAutochatRequest> {
    let mut bypass_prd = false;
    let mut kept_tokens: Vec<&str> = Vec::new();

    for token in rest.split_whitespace() {
        if token.eq_ignore_ascii_case(AUTOCHAT_NO_PRD_FLAG) {
            bypass_prd = true;
        } else {
            kept_tokens.push(token);
        }
    }

    let normalized = kept_tokens.join(" ");
    let explicit_count = normalized
        .split_whitespace()
        .next()
        .and_then(|value| value.parse::<usize>().ok())
        .is_some();
    let (agent_count, task) = parse_autochat_args(&normalized, default_agents, quick_demo_task)?;

    Some(ParsedAutochatRequest {
        agent_count,
        task: task.to_string(),
        bypass_prd,
        explicit_count,
    })
}
