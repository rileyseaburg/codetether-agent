use super::*;
/// List the standard coding prompts.
///
/// # Arguments
/// No arguments.
/// # Returns
/// Serialized prompt metadata.
/// # Errors
/// Response serialization errors.
///
/// # Examples
/// ```
/// let prompts = codetether_mcp_core::server_protocol::list_prompts().unwrap();
/// assert_eq!(prompts["prompts"][0]["name"], "code_review");
/// ```
pub fn list_prompts() -> Result<Value, JsonRpcError> {
    let result = ListPromptsResult {
        prompts: vec![
            McpPrompt {
                name: "code_review".to_string(),
                description: Some("Review code for issues and improvements".to_string()),
                arguments: vec![PromptArgument {
                    name: "file".to_string(),
                    description: Some("File to review".to_string()),
                    required: true,
                }],
            },
            McpPrompt {
                name: "explain_code".to_string(),
                description: Some("Explain what code does".to_string()),
                arguments: vec![PromptArgument {
                    name: "file".to_string(),
                    description: Some("File to explain".to_string()),
                    required: true,
                }],
            },
        ],
        next_cursor: None,
    };

    serde_json::to_value(result).map_err(|e| JsonRpcError::internal_error(e.to_string()))
}
/// Render one standard coding prompt.
///
/// # Arguments
/// `params` selects the prompt and supplies its file argument.
/// # Returns
/// The serialized prompt, defaulting the file to `file.rs`.
/// # Errors
/// Absent or invalid parameters, unknown prompt names or serialization errors.
///
/// # Examples
/// ```
/// assert!(codetether_mcp_core::server_protocol::get_prompt(None).is_err());
/// ```
pub fn get_prompt(params: Option<Value>) -> Result<Value, JsonRpcError> {
    let params: GetPromptParams = if let Some(p) = params {
        serde_json::from_value(p).map_err(|e| JsonRpcError::invalid_params(e.to_string()))?
    } else {
        return Err(JsonRpcError::invalid_params("Missing params"));
    };

    let result = match params.name.as_str() {
        "code_review" => super::prompt_review::build(&params),
        "explain_code" => super::prompt_explanation::build(&params),
        _ => return Err(JsonRpcError::method_not_found(&params.name)),
    };

    serde_json::to_value(result).map_err(|e| JsonRpcError::internal_error(e.to_string()))
}
