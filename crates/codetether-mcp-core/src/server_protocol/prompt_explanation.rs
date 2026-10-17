use super::*;
pub(super) fn build(params: &GetPromptParams) -> GetPromptResult {
    let file = params
        .arguments
        .get("file")
        .and_then(|v| v.as_str())
        .unwrap_or("file.rs");

    GetPromptResult {
        description: Some("Code explanation prompt".to_string()),
        messages: vec![PromptMessage {
            role: PromptRole::User,
            content: PromptContent::Text {
                text: format!(
                    "Please explain what this code does, including:\n\
                                     - Overall purpose\n\
                                     - Key functions and their roles\n\
                                     - Data flow\n\
                                     - Important algorithms used\n\n\
                                     File: {}",
                    file
                ),
            },
        }],
    }
}
