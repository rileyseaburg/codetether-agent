//! Report completion, timeout or step exhaustion from an instance story.
use super::*;
impl RalphLoop {
    pub(super) fn log_model_exit(
        story_id: &str,
        steps: usize,
        tool_calls: usize,
        exit_reason: AgentLoopExit,
    ) {
        // Check exit reason and log appropriately
        match exit_reason {
            AgentLoopExit::Completed => {
                info!(
                    "Ralph agent completed: {} steps, {} tool calls",
                    steps, tool_calls
                );
            }
            AgentLoopExit::MaxStepsReached => {
                warn!(
                    story_id = %story_id,
                    steps = steps,
                    tool_calls = tool_calls,
                    "Ralph sub-agent hit max steps limit - work may be incomplete"
                );
            }
            AgentLoopExit::TimedOut => {
                warn!(
                    story_id = %story_id,
                    steps = steps,
                    tool_calls = tool_calls,
                    "Ralph sub-agent timed out - work may be incomplete"
                );
            }
        }
    }
}
