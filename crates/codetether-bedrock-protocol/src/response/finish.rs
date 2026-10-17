//! Finish reason selection and invisible-output validation.
use super::*;
pub(super) fn finish(
    response: &ConverseResponse,
    content: &[ContentPart],
    has_tool_calls: bool,
) -> Result<FinishReason> {
    let finish_reason = if has_tool_calls {
        FinishReason::ToolCalls
    } else {
        crate::empty_guard::check_visible_output(content, response.stop_reason.as_deref())?;
        match response.stop_reason.as_deref() {
            Some("end_turn") | Some("stop") | Some("stop_sequence") => FinishReason::Stop,
            Some("max_tokens") => FinishReason::Length,
            Some("tool_use") => FinishReason::ToolCalls,
            Some("content_filtered") => FinishReason::ContentFilter,
            _ => FinishReason::Stop,
        }
    };
    Ok(finish_reason)
}
