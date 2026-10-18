//! Preserve instance-story exit reporting.
use codetether_swarm_core::agent_retry::AgentLoopExit;
use tracing::{info, warn};
pub(super) fn log(story_id: &str, steps: usize, tool_calls: usize, exit_reason: AgentLoopExit) {
    // Check exit reason and log appropriately
    match exit_reason {
        AgentLoopExit::Completed => {
            info!(target: "codetether_agent::ralph::ralph_loop::model_exit",
                "Ralph agent completed: {} steps, {} tool calls",
                steps, tool_calls
            );
        }
        AgentLoopExit::MaxStepsReached => {
            warn!(target: "codetether_agent::ralph::ralph_loop::model_exit",
                story_id = %story_id,
                steps = steps,
                tool_calls = tool_calls,
                "Ralph sub-agent hit max steps limit - work may be incomplete"
            );
        }
        AgentLoopExit::TimedOut => {
            warn!(target: "codetether_agent::ralph::ralph_loop::model_exit",
                story_id = %story_id,
                steps = steps,
                tool_calls = tool_calls,
                "Ralph sub-agent timed out - work may be incomplete"
            );
        }
    }
}
