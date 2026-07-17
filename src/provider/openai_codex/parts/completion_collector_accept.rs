impl CompletionCollector {
    fn accept(&mut self, chunk: StreamChunk) -> Result<()> {
        match chunk {
            StreamChunk::Text(delta) => self.text.push_str(&delta),
            StreamChunk::ToolCallStart { id, name } => self.start_tool(id, name),
            StreamChunk::ToolCallDelta {
                id,
                arguments_delta,
            } => {
                self.append_tool_arguments(id, arguments_delta);
            }
            StreamChunk::ToolCallEnd { .. } => {}
            StreamChunk::Thinking(value) => {
                if super::codex_reasoning::is_signature(&value) {
                    self.reasoning_signature = Some(value);
                }
            }
            StreamChunk::Done { usage } => {
                self.saw_done = true;
                if let Some(usage) = usage {
                    self.usage = usage;
                }
            }
            StreamChunk::Error(message) => anyhow::bail!(message),
        }
        Ok(())
    }
}
