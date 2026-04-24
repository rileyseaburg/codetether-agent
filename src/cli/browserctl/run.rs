use super::{BrowserCtlArgs, request};
use crate::tool::{Tool, browserctl::BrowserCtlTool};
use anyhow::{Result, bail};
use serde_json::Value;

pub async fn execute(args: BrowserCtlArgs) -> Result<()> {
    let tool = BrowserCtlTool::new();
    if request::needs_attach(&args.command) {
        call(&tool, request::attach(&args)).await?;
    }
    let result = call(&tool, request::command(&args)).await?;
    print_output(&result.output, args.json);
    Ok(())
}

async fn call(tool: &BrowserCtlTool, payload: Value) -> Result<crate::tool::ToolResult> {
    let result = tool.execute(payload).await?;
    if !result.success {
        bail!("{}", result.output);
    }
    Ok(result)
}

fn print_output(output: &str, _json: bool) {
    println!("{output}");
}
