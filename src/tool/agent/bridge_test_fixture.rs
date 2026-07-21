//! Test-only construction of observed remote-agent state.

use crate::tool::ToolResult;

pub(crate) fn record_remote_turn(name: &str, parent: &str, prompt: &str, output: &str) {
    let turn = super::super::message::remote::observation::begin(name, Some(parent), prompt);
    turn.complete(&ToolResult::success(output));
}
