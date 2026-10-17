//! Model-facing RustyRoad tool implementation.

use super::{Host, RustyRoadTool, run, schema};
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;

#[async_trait]
impl<H: Host> Tool for RustyRoadTool<H> {
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
        run::execute::<H>(args).await
    }
}
