//! Convert shared messages and tools to SDK request types.

use super::*;
impl OpenAIProvider {
    pub(super) fn convert_messages(
        messages: &[Message],
    ) -> Result<Vec<ChatCompletionRequestMessage>> {
        chat_messages::convert(messages)
    }
    pub(super) fn convert_tools(tools: &[ToolDefinition]) -> Result<Vec<ChatCompletionTools>> {
        let mut result = Vec::new();
        for tool in tools {
            result.push(ChatCompletionTools::Function(ChatCompletionTool {
                function: FunctionObjectArgs::default()
                    .name(&tool.name)
                    .description(&tool.description)
                    .parameters(tool.parameters.clone())
                    .build()?,
            }));
        }
        Ok(result)
    }
}
