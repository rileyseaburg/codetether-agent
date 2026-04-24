use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

use super::{errors, load, result, schema, task};
use crate::tool::{Tool, ToolResult};

use super::input::TetherScriptPluginInput;
use super::tool::TetherScriptPluginTool;

#[async_trait]
impl Tool for TetherScriptPluginTool {
    fn id(&self) -> &str {
        "tetherscript_plugin"
    }

    fn name(&self) -> &str {
        "TetherScript Plugin"
    }

    fn description(&self) -> &str {
        "Execute a TetherScript plugin hook from inline source or a `.tether`/legacy `.kl` file path."
    }

    fn parameters(&self) -> Value {
        schema::parameters()
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        execute_tetherscript(self, args).await
    }
}

async fn execute_tetherscript(tool: &TetherScriptPluginTool, args: Value) -> Result<ToolResult> {
    let input: TetherScriptPluginInput = match serde_json::from_value(args) {
        Ok(input) => input,
        Err(error) => return Ok(errors::invalid_params(tool.id(), error)),
    };
    if !input.has_source() && !input.has_path() {
        return Ok(errors::missing_source(tool.id()));
    }

    let (source_name, source) = match load::source(&input, tool.root()).await {
        Ok(source) => source,
        Err(error) => return Ok(ToolResult::error(error.to_string())),
    };
    let request = task::TetherScriptRun::new(source_name, source, input);
    Ok(result::from_run(task::run(request).await?))
}
