//! FunctionGemma chat-template construction.

use super::prompt_tools::{tool_lines, tools_json};
use crate::provider::ToolDefinition;

/// Assistant text is truncated to this many bytes: intent is expressed early,
/// and the rest is often hallucinated output or markdown formatting.
const MAX_ASSISTANT_BYTES: usize = 500;

/// Serialize tool definitions into FunctionGemma's expected chat template.
///
/// The prompt frames FunctionGemma as a tool-call extractor: given an LLM's text
/// response that *describes* tool usage, produce the corresponding structured
/// `<tool_call>` blocks.
pub(super) fn build_functiongemma_prompt(assistant_text: &str, tools: &[ToolDefinition]) -> String {
    let tools_json = tools_json(tools);
    let tool_lines = tool_lines(tools);
    let truncated = truncate(assistant_text);

    format!(
        "<start_of_turn>system\n\
         You are a function calling AI model. You are provided with function \
         signatures within <tools></tools> XML tags. You may call one or more \
         functions to assist with the user query. Don't make assumptions about \
         what values to plug into functions.\n\n\
         <tools>\n{tools_json}\n</tools>\n\n\
         Other available tools:\n{tool_lines}\n\n\
         For each function call return a JSON object with function name and \
         arguments within <tool_call></tool_call> XML tags as follows:\n\
         <tool_call>\n{{\"name\": \"function_name\", \"arguments\": {{\"arg1\": \"value1\"}}}}\n</tool_call>\n\
         <end_of_turn>\n\
         <start_of_turn>user\n\
         The following is an AI assistant's response. It describes wanting to \
         use tools but expressed them as text instead of structured calls. \
         Extract the tool calls the assistant intended to make:\n\n\
         {truncated}\n\
         <end_of_turn>\n\
         <start_of_turn>model\n"
    )
}

/// Clip `text` to [`MAX_ASSISTANT_BYTES`] on a character boundary.
fn truncate(text: &str) -> &str {
    if text.len() <= MAX_ASSISTANT_BYTES {
        return text;
    }
    let mut end = MAX_ASSISTANT_BYTES;
    while end > 0 && !text.is_char_boundary(end) {
        end -= 1;
    }
    &text[..end]
}
