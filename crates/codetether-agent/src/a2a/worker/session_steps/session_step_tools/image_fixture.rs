//! Mock image tool for worker execution tests; no provider involved.
use crate::tool::{Tool, ToolResult};
use serde_json::{Value, json};
pub(super) const IMAGE: &str = "data:image/png;base64,aW1hZ2U=";
pub(super) const OUTPUT: &str = "raw output\n  unchanged: λ\n";
pub(super) struct ImageTool;
#[async_trait::async_trait]
impl Tool for ImageTool {
    fn id(&self) -> &str {
        "read"
    }
    fn name(&self) -> &str {
        self.id()
    }
    fn description(&self) -> &str {
        "Mock image result"
    }
    fn parameters(&self) -> Value {
        json!({"type": "object"})
    }
    async fn execute(&self, input: Value) -> anyhow::Result<ToolResult> {
        Ok(ToolResult {
            output: OUTPUT.into(),
            success: input["success"].as_bool().unwrap(),
            metadata: [(
                "image_data_url".into(),
                json!([
                    {"data_url": IMAGE, "mime_type": "image/png"}, IMAGE
                ]),
            )]
            .into(),
        })
    }
}
