//! Tool metadata and execution entry point.
use super::{MultiEditTool, Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use codetether_morph_backend::Editor;
use serde_json::Value;
#[async_trait]
impl<E: Editor> Tool for MultiEditTool<E> {
    fn id(&self) -> &str {
        "multiedit"
    }

    fn name(&self) -> &str {
        "Multi Edit"
    }

    fn description(&self) -> &str {
        "Apply multiple file edits atomically. Validates all edits, then writes all changes. \
         If any edit fails validation, no files are modified. Each edit replaces old_string \
            with new_string in the given file. Morph backend is available only when explicitly \
            enabled with CODETETHER_MORPH_TOOL_BACKEND=1; instruction/update can guide Morph \
            behavior per edit."
    }
    fn parameters(&self) -> Value {
        super::schema::parameters()
    }
    async fn execute(&self, params: Value) -> Result<ToolResult> {
        super::execute::execute(params, &self.editor).await
    }
}
