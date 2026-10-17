//! Request identity inside FunctionGemma's existing formatted system turn.

/// Refresh only system metadata, preserving the tool-extraction conversation.
///
/// # Arguments
/// * `prompt` - Formatted model prompt.
/// * `loaded_model` - Identity of the loaded native model.
///
/// # Returns
/// The formatted or selected result described above.
///
/// # Examples
/// ```
/// # use codetether_provider_core::{ToolDefinition, CompletionResponse};
/// # fn example(prompt: &str, loaded_model: &str) {
/// let _result = codetether_cognition_core::tool_routing::with_identity(prompt, loaded_model);
/// # }
/// ```
pub fn with_identity(prompt: &str, loaded_model: &str) -> String {
    const START: &str = "<start_of_turn>system\n";
    const END: &str = "<end_of_turn>";
    let system = codetether_provider_core::request_identity::system_prompt;
    if let Some(body) = prompt.strip_prefix(START)
        && let Some((caller, rest)) = body.split_once(END)
    {
        return format!(
            "{START}{}{END}{rest}",
            system(caller, "candle", loaded_model)
        );
    }
    format!(
        "{START}{}{END}\n{prompt}",
        system("", "candle", loaded_model)
    )
}
