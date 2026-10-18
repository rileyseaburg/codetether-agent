//! Preserve visible prose while collecting registered textual tool calls.
use crate::{extract_bare_json_tool_call, extract_markup_tool_calls};
use codetether_provider_core::ContentPart;
use std::collections::HashSet;
pub(super) fn append(
    text: &str,
    allowed_tools: &HashSet<&str>,
    rewritten: &mut Vec<ContentPart>,
    parsed_calls: &mut Vec<(String, String)>,
) {
    let (cleaned, calls) = extract_markup_tool_calls(text);
    let mut text_was_a_call = false;
    if calls.is_empty()
        && let Some((name, arguments)) = extract_bare_json_tool_call(&cleaned, allowed_tools)
    {
        parsed_calls.push((name, arguments));
        text_was_a_call = true;
    }
    for (name, arguments) in calls {
        if allowed_tools.contains(name.as_str()) {
            parsed_calls.push((name, arguments));
        } else {
            tracing::warn!(target: "codetether_agent::session::helper::markup",tool = %name, "Ignoring unknown <tool_call> tool name");
        }
    }

    if !text_was_a_call && !cleaned.trim().is_empty() {
        rewritten.push(ContentPart::Text {
            text: cleaned.trim().to_string(),
        });
    }
}
