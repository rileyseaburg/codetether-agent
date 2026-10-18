//! Native insertion of shared thinking-only continuation guidance.
use super::super::Runner;
use crate::provider::CompletionResponse;
pub(super) fn continue_instead(runner: &mut Runner<'_>, response: &CompletionResponse) -> bool {
    let Some(text) =
        codetether_prompt_loop::response::thinking_only_nudge(&response.message.content)
    else {
        return false;
    };
    super::nudge::add(runner, text);
    true
}
