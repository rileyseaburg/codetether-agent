//! Incorporation of completed items into retry state and request context.

use crate::provider::{CompletionRequest, CompletionResponse, ContentPart};

pub(super) fn run(
    state: &mut super::State,
    outcome: &super::super::super::outcome::DrainOutcome,
    request: &mut CompletionRequest,
) -> Option<CompletionResponse> {
    if outcome.completed.is_empty() {
        return None;
    }
    let completed = outcome.completed.clone();
    request
        .messages
        .push(super::response::message(completed.clone()));
    let has_tool = completed
        .iter()
        .any(|part| matches!(part, ContentPart::ToolCall { .. }));
    state.content.extend(completed);
    has_tool.then(|| super::response::build(state.content.clone(), super::response::usage(outcome)))
}
