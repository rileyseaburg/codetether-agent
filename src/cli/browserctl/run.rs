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

fn print_output(output: &str, json: bool) {
    println!("{}", format_output(output, json));
}

pub(super) fn format_output(output: &str, json: bool) -> String {
    let Ok(value) = serde_json::from_str::<Value>(output) else {
        return output.to_string();
    };
    if json {
        serde_json::to_string(&value).unwrap_or_else(|_| output.to_string())
    } else {
        serde_json::to_string_pretty(&value).unwrap_or_else(|_| output.to_string())
    }
}
