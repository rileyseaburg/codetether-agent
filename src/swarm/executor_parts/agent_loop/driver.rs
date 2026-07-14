//! Provider-turn driver for a reusable sub-agent loop.

use super::super::{AgentLoopExit, loop_step};
use super::{limits, output, request, state::State, thinking, tools, turn};
use anyhow::Result;

pub(super) async fn execute(state: &mut State) -> Result<AgentLoopExit> {
    loop {
        if let Some(exit) = limits::reached(state) {
            return Ok(exit);
        }
        state.steps += 1;
        crate::swarm::token_truncate::truncate_messages_to_fit(
            &mut state.messages,
            crate::swarm::token_truncate::DEFAULT_CONTEXT_LIMIT,
        );
        let response = match request::execute(state).await? {
            request::Outcome::Response(response) => response,
            request::Outcome::TimedOut => return Ok(AgentLoopExit::TimedOut),
        };
        let response =
            crate::session::helper::markup::normalize_textual_tool_calls(response, &state.tools);
        let turn = turn::parse(response);
        thinking::publish(state, &turn.thinking);
        output::record(state, &turn.text);
        let calls = turn
            .tools
            .iter()
            .map(|call| (call.id.clone(), call.name.clone(), call.arguments.clone()))
            .collect::<Vec<_>>();
        loop_step::log_tool_calls(state.steps, &calls);
        let has_tools = !turn.tools.is_empty();
        state.messages.push(turn.message);
        match loop_step::after_turn(turn.finish, has_tools) {
            loop_step::AfterTurn::Execute => {
                if tools::execute(state, turn.tools).await {
                    return Ok(AgentLoopExit::TimedOut);
                }
            }
            loop_step::AfterTurn::Continue => continue,
            loop_step::AfterTurn::Finish => return Ok(AgentLoopExit::Completed),
        }
    }
}
