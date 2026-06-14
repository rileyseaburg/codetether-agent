//! Loop-control constants shared by the agentic prompt loops.
//!
//! The session's `prompt*` methods run an agentic loop that repeatedly calls
//! the LLM and executes any tool calls it emits. The constants here tune the
//! guardrails that detect and break pathological loops (e.g. the model keeps
//! running the same tool, keeps promising to call a tool without ever doing
//! it, etc.).
//!
//! The numeric guardrails can be overridden per-invocation via environment
//! variables (`CODETETHER_MAX_TOOL_CALLS`,
//! `CODETETHER_MAX_CONSECUTIVE_SAME_TOOL`,
//! `CODETETHER_MAX_STEPS_WITHOUT_PROGRESS`) — mirroring how
//! [`crate::config::guardrails::CostGuardrails`] reads its cost caps.

/// Reminder sent when the build-mode agent responds with a plan-only answer
/// despite the user asking for a file change.
pub(crate) const BUILD_MODE_TOOL_FIRST_NUDGE: &str = "Build mode policy reminder: execute directly. \
Start by calling at least one appropriate tool now (or emit <tool_call> markup for non-native \
tool providers). Do not ask for permission and do not provide a plan-only response.";

/// Maximum number of "tool-first" retries before we surface an error to the
/// caller instead of nudging again.
pub(crate) const BUILD_MODE_TOOL_FIRST_MAX_RETRIES: u8 = 2;

/// Maximum number of retries when the model claims it is about to use a tool
/// but never actually emits a tool call.
pub(crate) const NATIVE_TOOL_PROMISE_RETRY_MAX_RETRIES: u8 = 1;

/// Threshold of consecutive codesearch "no matches" results before we nudge
/// the model away from brute-force variant retries.
pub(crate) const MAX_CONSECUTIVE_CODESEARCH_NO_MATCHES: u32 = 5;

/// Maximum retries for post-edit validation (build/lint/test) failures before
/// we give up and surface the report to the caller.
pub(crate) const POST_EDIT_VALIDATION_MAX_RETRIES: u8 = 3;

/// Default for [`max_consecutive_same_tool`].
pub(crate) const DEFAULT_MAX_CONSECUTIVE_SAME_TOOL: u32 = 3;

/// Default for [`max_total_tool_calls`]. Sized as a stuck-model backstop,
/// not a working budget: legitimate implementation turns routinely need
/// >100 calls, and `--max-steps` (default 250) already bounds the loop.
pub(crate) const DEFAULT_MAX_TOTAL_TOOL_CALLS: u32 = 200;

/// Default for [`max_steps_without_progress`].
pub(crate) const DEFAULT_MAX_STEPS_WITHOUT_PROGRESS: u32 = 15;

/// Maximum number of consecutive identical tool calls allowed before the loop
/// forces a final answer. Overridable via
/// `CODETETHER_MAX_CONSECUTIVE_SAME_TOOL`.
pub(crate) fn max_consecutive_same_tool() -> u32 {
    env_u32("CODETETHER_MAX_CONSECUTIVE_SAME_TOOL").unwrap_or(DEFAULT_MAX_CONSECUTIVE_SAME_TOOL)
}

/// Absolute ceiling on total tool calls within a single agentic turn.
/// Beyond this the loop terminates with an error summarising what happened.
/// Overridable via `CODETETHER_MAX_TOOL_CALLS`.
pub(crate) fn max_total_tool_calls() -> u32 {
    env_u32("CODETETHER_MAX_TOOL_CALLS").unwrap_or(DEFAULT_MAX_TOTAL_TOOL_CALLS)
}

/// Steps with no file mutations (write/edit/bash) before we nudge the model
/// to either make progress or provide a final answer. Overridable via
/// `CODETETHER_MAX_STEPS_WITHOUT_PROGRESS`.
pub(crate) fn max_steps_without_progress() -> u32 {
    env_u32("CODETETHER_MAX_STEPS_WITHOUT_PROGRESS").unwrap_or(DEFAULT_MAX_STEPS_WITHOUT_PROGRESS)
}

/// Parse a positive integer from the environment. Zero is treated as
/// unset — every guardrail here would otherwise trip on the first step.
fn env_u32(key: &str) -> Option<u32> {
    std::env::var(key)
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .filter(|v: &u32| *v > 0)
}

/// Nudge sent when the model keeps trying punctuation/casing variants of the
/// same identifier via codesearch.
pub(crate) const CODESEARCH_THRASH_NUDGE: &str = "Stop brute-force codesearch variant retries. \
You already got repeated \"No matches found\" results. Do not try punctuation/casing/underscore \
variants of the same token again. Either switch to a broader strategy (e.g., inspect likely files \
directly) or conclude the identifier is absent and continue with the best available evidence.";

/// Nudge sent when the model describes an upcoming tool call in prose instead
/// of actually emitting one.
pub(crate) const NATIVE_TOOL_PROMISE_NUDGE: &str = "You said you would use tools. Do not describe the tool \
call or promise a next step. Emit the actual tool call now. If native tool calling fails, emit a \
<tool_call> JSON block immediately instead of prose.";

/// Message sent when the loop-detection guard is forcing a final answer.
pub(crate) const FORCE_FINAL_ANSWER_NUDGE: &str = "STOP using tools. Provide your final answer NOW \
in plain text based on the tool results you already received. Do NOT output any <tool_call> blocks.";

/// Nudge when the model is making many tool calls but not writing any files.
pub(crate) const NO_PROGRESS_NUDGE: &str = "You have made many tool calls without writing or editing \
any files. If you are still investigating, that is fine — but if you have gathered enough information, \
stop reading/searching and start implementing. Provide a final answer if you cannot complete the task.";

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    /// Serialise env mutation across tests in this module.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn with_env<F: FnOnce()>(key: &str, value: Option<&str>, f: F) {
        let _guard = ENV_LOCK.lock().unwrap();
        // SAFETY: serialised by ENV_LOCK above.
        unsafe {
            match value {
                Some(v) => std::env::set_var(key, v),
                None => std::env::remove_var(key),
            }
        }
        f();
        // SAFETY: serialised by ENV_LOCK above.
        unsafe { std::env::remove_var(key) };
    }

    #[test]
    fn tool_call_budget_defaults_when_unset() {
        with_env("CODETETHER_MAX_TOOL_CALLS", None, || {
            assert_eq!(max_total_tool_calls(), DEFAULT_MAX_TOTAL_TOOL_CALLS);
        });
    }

    #[test]
    fn tool_call_budget_env_override() {
        with_env("CODETETHER_MAX_TOOL_CALLS", Some("500"), || {
            assert_eq!(max_total_tool_calls(), 500);
        });
    }

    #[test]
    fn zero_and_garbage_fall_back_to_default() {
        with_env("CODETETHER_MAX_TOOL_CALLS", Some("0"), || {
            assert_eq!(max_total_tool_calls(), DEFAULT_MAX_TOTAL_TOOL_CALLS);
        });
        with_env("CODETETHER_MAX_TOOL_CALLS", Some("lots"), || {
            assert_eq!(max_total_tool_calls(), DEFAULT_MAX_TOTAL_TOOL_CALLS);
        });
    }
}
