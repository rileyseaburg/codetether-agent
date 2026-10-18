//! Normalize and classify calls before parallel scheduling.
use codetether_edit_policy::{detect_stub_in_tool_input, normalize_tool_call_for_execution};
use codetether_prompt_policy::is_interactive_tool;

pub(super) fn is_eligible(raw_name: &str, raw_input: &serde_json::Value) -> bool {
    let (name, input) = normalize_tool_call_for_execution(raw_name, raw_input);
    codetether_tool_core::readonly::is_read_only(&name)
        && !is_interactive_tool(&name)
        && detect_stub_in_tool_input(&name, &input).is_none()
}
