use super::*;
/// Run provider turns until completion, a limit, cancellation or execution error.
///
/// # Arguments
/// * `runtime` - Host state and side-effect adapters.
///
/// # Returns
/// The terminal reason for the agent loop.
///
/// # Errors
/// Returns errors from readiness, provider requests or tool execution.
///
/// # Examples
/// ```no_run
/// # async fn example(host: &mut impl codetether_swarm_core::agent_loop::AgentRuntime) -> anyhow::Result<()> {
/// let _exit = codetether_swarm_core::agent_loop::execute(host).await?;
/// # Ok(())
/// # }
/// ```
pub async fn execute(runtime: &mut impl AgentRuntime) -> Result<AgentLoopExit> {
    loop {
        runtime.ready().await?;
        runtime.apply_input();
        if let Some(exit) = runtime.limit_reached() {
            return Ok(exit);
        }
        runtime.advance();
        let response = match runtime.request().await? {
            RequestOutcome::Response(response) => response,
            RequestOutcome::TimedOut => return Ok(AgentLoopExit::TimedOut),
        };
        let response = runtime.normalize(response);
        let turn = parse_turn(response);
        runtime.publish_thinking(&turn.thinking);
        runtime.record_output(&turn.text);
        let calls = turn
            .tools
            .iter()
            .map(|call| (call.id.clone(), call.name.clone(), call.arguments.clone()))
            .collect::<Vec<_>>();
        step::log_tool_calls(runtime.steps(), &calls);
        let has_tools = !turn.tools.is_empty();
        runtime.append_message(turn.message);
        match step::after_turn(turn.finish, has_tools) {
            step::AfterTurn::Execute => {
                if runtime.execute_tools(turn.tools).await? {
                    return Ok(AgentLoopExit::TimedOut);
                }
            }
            step::AfterTurn::Continue => continue,
            step::AfterTurn::Finish if runtime.apply_input() => continue,
            step::AfterTurn::Finish => return Ok(AgentLoopExit::Completed),
        }
    }
}
