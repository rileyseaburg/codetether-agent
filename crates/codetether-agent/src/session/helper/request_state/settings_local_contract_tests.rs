//! Textual tool contract regression for providers without native tool channels.
use super::{model_supports_tools, system_prompt_for, tool};
#[test]
fn local_cuda_system_prompt_carries_textual_tool_contract() {
    let tools = vec![tool("bash"), tool("read")];
    let prompt = system_prompt_for(
        "local_cuda",
        model_supports_tools("local_cuda", "nemotron-nano-8b-v1"),
        &tools,
        std::path::Path::new("."),
        false,
    );
    assert!(
        prompt.contains("<tool_call>"),
        "textual tool contract missing"
    );
    assert!(prompt.contains("bash"), "tool names missing from prompt");
}
