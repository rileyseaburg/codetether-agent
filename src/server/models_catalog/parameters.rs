//! OpenRouter-style supported parameter names.

pub(crate) fn supported_parameters(model: &crate::provider::ModelInfo) -> Vec<String> {
    let mut params = base_parameters();
    if model.supports_tools {
        params.extend(["tools", "tool_choice"]);
    }
    params.into_iter().map(str::to_string).collect()
}

fn base_parameters() -> Vec<&'static str> {
    vec!["max_tokens", "temperature", "top_p", "stop"]
}
