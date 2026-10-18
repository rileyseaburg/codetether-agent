//! Interpret relay aliases and validate execution-mode restrictions.
use crate::{
    RelayRequest, command_with_optional_args, is_easy_go_command, normalize_cli_go_command,
};
use codetether_autochat_core::{
    AUTOCHAT_DEFAULT_AGENTS, AUTOCHAT_MAX_AGENTS, AUTOCHAT_QUICK_DEMO_TASK,
};
/// Parse a relay request while leaving ordinary prompts untouched.
/// # Arguments
/// * `message` - Input command or ordinary prompt.
/// * `jsonl_output` - Whether interactive approval is unavailable.
/// # Returns
/// A normalized relay request or none when the input is not a relay command.
/// # Errors
/// Returns existing usage, task, approval-mode or team-size validation errors.
/// # Examples
/// ```
/// assert!(codetether_run_command::parse_relay_request("ordinary prompt", false).unwrap().is_none());
/// assert!(codetether_run_command::parse_relay_request("/go repair", true).is_err());
/// ```
pub fn parse_relay_request(
    message: &str,
    jsonl_output: bool,
) -> anyhow::Result<Option<RelayRequest>> {
    let easy_go_requested = is_easy_go_command(message);
    let normalized = normalize_cli_go_command(message);
    if let Some(rest) = command_with_optional_args(&normalized, "/autochat") {
        let Some(parsed) = codetether_autochat_core::parse_autochat_request(
            rest,
            AUTOCHAT_DEFAULT_AGENTS,
            AUTOCHAT_QUICK_DEMO_TASK,
        ) else {
            anyhow::bail!(
                "Usage: /autochat [count] [--no-prd] <task>\nEasy mode: /go <task>\ncount range: 2-{} (default: {})",
                AUTOCHAT_MAX_AGENTS,
                AUTOCHAT_DEFAULT_AGENTS
            );
        };
        Ok(Some(crate::parsed::resolve(
            parsed,
            easy_go_requested,
            jsonl_output,
        )?))
    } else {
        Ok(None)
    }
}
