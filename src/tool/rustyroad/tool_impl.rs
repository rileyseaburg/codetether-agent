//! Model-facing RustyRoad tool implementation.

use super::{RustyRoadTool, run, schema};
use crate::tool::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
impl Tool for RustyRoadTool {
    fn id(&self) -> &str {
        "rustyroad"
    }
    fn name(&self) -> &str {
        "RustyRoad"
    }
    fn description(&self) -> &str {
        "First-party RustyRoad database and project tools. Use list_tools, then \
         call_tool with an advertised rustyroad_* name and its arguments. \
         Always specify cwd; environment defaults to dev, never ambient prod. \
         Supports queries, schema, migrations, configuration and project inspection. \
         Requires cargo install rustyroad --locked --bin rustyroad-mcp."
    }
    fn parameters(&self) -> Value {
        schema::parameters()
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        run::execute(args).await
    }
}
