use super::*;

pub(super) fn build(
    spec: &ExecutionSpec<'_>,
    output: String,
    steps: usize,
    tools: usize,
    exit: AgentLoopExit,
    retries: u32,
    elapsed: Duration,
) -> (SubTaskResult, SubTaskStatus) {
    let output = deliverable::verify_output(spec.instruction, &output);
    let error = match exit {
        AgentLoopExit::Completed => deliverable::deliverable_error(&output),
        AgentLoopExit::MaxStepsReached => {
            Some(format!("Sub-agent hit max steps ({})", spec.max_steps))
        }
        AgentLoopExit::TimedOut => {
            Some(format!("Sub-agent timed out after {}s", spec.timeout_secs))
        }
    };
    let success = error.is_none();
    let status = match exit {
        AgentLoopExit::Completed if success => SubTaskStatus::Completed,
        AgentLoopExit::TimedOut => SubTaskStatus::TimedOut,
        AgentLoopExit::Completed | AgentLoopExit::MaxStepsReached => SubTaskStatus::Failed,
    };
    (
        make::make(spec, success, output, steps, tools, error, retries, elapsed),
        status,
    )
}
