//! Dispatch tools, accept final text, or append a continuation.

use crate::{
    context_trace::ContextTrace,
    router::host::RouterHost,
    traits::{LlmMessage, LlmResponse},
};
pub(super) fn process(
    host: &mut dyn RouterHost,
    response: &LlmResponse,
    conversation: &mut Vec<LlmMessage>,
    trace: &mut ContextTrace,
    summary_mode: bool,
    iterations: usize,
) -> Option<String> {
    if let Some(a) =
        super::tool_dispatch::try_tool_calls(host, response, conversation, trace, summary_mode)
    {
        return Some(a);
    }
    if let Some(a) = super::text_path::try_text_path(response, summary_mode, iterations) {
        return Some(a);
    }
    super::text_path::push_continuation(conversation, response);
    None
}
