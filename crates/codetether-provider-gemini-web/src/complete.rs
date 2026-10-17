//! Buffered completion assembly.
use crate::*;
impl GeminiWebProvider {
    pub(crate) async fn complete_response(
        &self,
        request: CompletionRequest,
    ) -> Result<CompletionResponse> {
        let prompt = prompt::render(&request.messages, &request.tools)
            .context("Failed to render Gemini Web prompt")?;

        let (cleaned_text, parsed_tool_calls) = self
            .ask_validated(&prompt, &request.model, &request.tools, &request.messages)
            .await
            .context("Gemini Web completion failed")?;
        let usage = usage::estimate(&prompt, &cleaned_text, &parsed_tool_calls);
        let mut content = response_text::content_parts(&cleaned_text);

        for (idx, (name, arguments)) in parsed_tool_calls.iter().enumerate() {
            let ts = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis();
            content.push(ContentPart::ToolCall {
                id: format!("gwc_{ts}_{idx}"),
                name: name.clone(),
                arguments: arguments.clone(),
                thought_signature: None,
            });
        }

        let finish_reason = if parsed_tool_calls.is_empty() {
            FinishReason::Stop
        } else {
            tracing::info!(
                model = %request.model,
                num_calls = parsed_tool_calls.len(),
                "Parsed tool calls from Gemini web text response"
            );
            FinishReason::ToolCalls
        };

        Ok(CompletionResponse {
            message: Message {
                role: Role::Assistant,
                content,
            },
            usage,
            finish_reason,
        })
    }
}
