impl CompletionCollector {
    fn finish(self) -> Result<CompletionResponse> {
        if !self.saw_done {
            anyhow::bail!("temporary provider availability issue; retry the request");
        }
        let mut content = Vec::new();
        if !self.text.is_empty() {
            content.push(ContentPart::Text { text: self.text });
        }
        content.extend(self.tools.into_iter().map(|tool| ContentPart::ToolCall {
            id: tool.id,
            name: tool.name,
            arguments: tool.arguments,
            thought_signature: None,
        }));
        let has_tools = content
            .iter()
            .any(|part| matches!(part, ContentPart::ToolCall { .. }));
        Ok(CompletionResponse {
            message: Message {
                role: Role::Assistant,
                content,
            },
            usage: self.usage,
            finish_reason: if has_tools {
                FinishReason::ToolCalls
            } else {
                FinishReason::Stop
            },
        })
    }
}
