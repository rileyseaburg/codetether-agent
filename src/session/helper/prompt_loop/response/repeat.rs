//! Detection of repeated tool batches that should force an answer.

use super::super::super::loop_constants as limits;
use super::super::Runner;
use crate::provider::{CompletionResponse, ContentPart};
type ToolCall = (String, String, serde_json::Value);

/// Detects repeated tool batches and inserts a final-answer nudge.
pub(super) fn force_answer(
    runner: &mut Runner<'_>,
    step: usize,
    response: &CompletionResponse,
    calls: &[ToolCall],
) -> bool {
    let mut signatures: Vec<String> = calls
        .iter()
        .map(|(_, name, args)| format!("{name}:{args}"))
        .collect();
    signatures.sort();
    let signature = signatures.join("|");
    if runner.progress.last_tool_signature.as_deref() == Some(&signature) {
        runner.progress.repeated_tools += 1;
    } else {
        runner.progress.repeated_tools = 1;
        runner.progress.last_tool_signature = Some(signature);
    }
    let force = runner.progress.repeated_tools > limits::MAX_CONSECUTIVE_SAME_TOOL
        || (!runner.model.supports_tools && step >= 3);
    if !force {
        return false;
    }
    let mut assistant = response.message.clone();
    assistant
        .content
        .retain(|part| !matches!(part, ContentPart::ToolCall { .. }));
    if !assistant.content.is_empty() {
        runner.session.add_message(assistant);
    }
    super::build_guard::nudge(runner, limits::FORCE_FINAL_ANSWER_NUDGE);
    true
}
