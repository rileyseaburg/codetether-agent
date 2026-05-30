//! Compact tool-schema helpers for context-window fallback.

#[derive(Default)]
pub(super) struct CountingWriter {
    pub(super) bytes: usize,
}

impl std::io::Write for CountingWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.bytes += buf.len();
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub(super) fn compact_worker_tool_definitions(
    tools: &[crate::provider::ToolDefinition],
) -> Vec<crate::provider::ToolDefinition> {
    tools
        .iter()
        .map(|tool| crate::provider::ToolDefinition {
            name: tool.name.clone(),
            description: tool.description.clone(),
            parameters: serde_json::json!({ "type": "object", "additionalProperties": true }),
        })
        .collect()
}

pub(super) fn tool_schema_bytes(tools: &[crate::provider::ToolDefinition]) -> usize {
    let mut writer = CountingWriter::default();
    serde_json::to_writer(&mut writer, tools)
        .map(|_| writer.bytes)
        .unwrap_or(0)
}
