//! Classify a finished worker while retaining its identity and model diagnostics.
use crate::{
    model_selection::ModelSelection,
    task_input::TaskInput,
    task_result::{self, TaskResult},
};
use codetether_swarm_core::agent_retry::AgentLoopExit;
pub(super) fn finish(
    task: TaskInput,
    task_id: String,
    model_selection: ModelSelection,
    (output, steps, tool_calls, exit): (String, usize, usize, AgentLoopExit),
) -> TaskResult {
    let error = task_result::failure(exit, &output);
    let success = error.is_none();

    TaskResult {
        task_id,
        task_name: task.name,
        success,
        output,
        error,
        steps,
        tool_calls,
        model: model_selection,
    }
}
