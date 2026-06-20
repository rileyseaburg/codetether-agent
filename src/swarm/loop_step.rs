//! Per-turn continuation decision for the swarm sub-agent loop.
//!
//! Cerebras/GLM and several OpenAI-compatible providers report `Stop` even
//! when they emit tool calls, so the loop must key off the *presence* of tool
//! calls rather than `finish_reason`. Keying off `finish_reason` previously
//! caused sub-agents to exit after emitting a plan without executing the
//! queued tools.

use crate::provider::FinishReason;

/// What the agentic loop should do after an assistant turn.
pub(super) enum AfterTurn {
    /// Tool calls are pending: execute them and continue.
    Execute,
    /// Response was truncated mid-thought: re-prompt to let it finish.
    Continue,
    /// Genuine stop with no pending work: the sub-agent is done.
    Finish,
}

/// Decide the next loop action from the stop reason and tool-call presence.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::FinishReason;
/// use codetether_agent::swarm::executor::loop_step::{after_turn, AfterTurn};
///
/// assert!(matches!(after_turn(FinishReason::Stop, true), AfterTurn::Execute));
/// assert!(matches!(after_turn(FinishReason::Length, false), AfterTurn::Continue));
/// assert!(matches!(after_turn(FinishReason::Stop, false), AfterTurn::Finish));
/// ```
pub fn after_turn(finish: FinishReason, has_tool_calls: bool) -> AfterTurn {
    if has_tool_calls {
        AfterTurn::Execute
    } else if finish == FinishReason::Length {
        AfterTurn::Continue
    } else {
        AfterTurn::Finish
    }
}

/// Emit a structured log of the tool calls the sub-agent requested this turn.
///
/// No-op when `tool_calls` is empty.
pub(super) fn log_tool_calls(step: usize, tool_calls: &[(String, String, String)]) {
    if tool_calls.is_empty() {
        return;
    }
    let names: Vec<&str> = tool_calls.iter().map(|(_, name, _)| name.as_str()).collect();
    tracing::info!(step, num_tool_calls = tool_calls.len(), tools = ?names,
        "Sub-agent requesting tool calls");
}