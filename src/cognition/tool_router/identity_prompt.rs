//! Request identity inside FunctionGemma's existing formatted system turn.

/// Refresh only system metadata, preserving the tool-extraction conversation.
pub(super) fn with_identity(prompt: &str, loaded_model: &str) -> String {
    const START: &str = "<start_of_turn>system\n";
    const END: &str = "<end_of_turn>";
    let system = crate::provider::metrics::identity::system_prompt;
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
