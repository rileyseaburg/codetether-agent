use super::*;
pub(super) fn build(params: &GetPromptParams) -> GetPromptResult {
    let file = params
        .arguments
        .get("file")
        .and_then(|v| v.as_str())
        .unwrap_or("file.rs");

    GetPromptResult {
        description: Some("Code review prompt".to_string()),
        messages: vec![PromptMessage {
            role: PromptRole::User,
            content: PromptContent::Text {
                text: format!(
                    "Please review the following code for:\n\
                                     - Bugs and potential issues\n\
                                     - Performance concerns\n\
                                     - Code style and best practices\n\
                                     - Security vulnerabilities\n\n\
                                     File: {}",
                    file
                ),
            },
        }],
    }
}
