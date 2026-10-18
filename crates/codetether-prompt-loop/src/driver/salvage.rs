use super::{Guards, ParsedCall};
use codetether_provider_core::CompletionResponse;
pub(super) fn apply(
    host: &impl Guards,
    text: &str,
    truncated: &[(String, String)],
    response: &mut CompletionResponse,
    calls: &mut Vec<ParsedCall>,
) {
    let view = host.provider_view();
    codetether_tool_recovery::salvage_prose_tool_call(
        view.provider,
        view.model,
        text,
        view.tools,
        truncated,
        response,
        calls,
    );
}
