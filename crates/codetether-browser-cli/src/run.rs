//! Execute browser-control requests and render their outputs.

use super::{BrowserCtlArgs, BrowserCtlCommand, offline, request};
use anyhow::{Result, bail};
use codetether_browser_tool::BrowserCtlTool;
use codetether_tool_core::Tool;
use serde_json::Value;

/// Execute browser-control or offline commands through the shared services.
///
/// # Arguments
/// * `args` - Parsed browser options and operation.
/// # Returns
/// Unit after printing the requested output.
/// # Errors
/// Returns tool, offline-operation or request errors.
/// # Examples
/// ```no_run
/// # async fn example(args: codetether_browser_cli::BrowserCtlArgs) -> anyhow::Result<()> {
/// codetether_browser_cli::execute(args).await
/// # }
/// ```
pub async fn execute(args: BrowserCtlArgs) -> Result<()> {
    if let BrowserCtlCommand::Offline { cmd } = &args.command {
        return offline::execute(cmd, args.json);
    }
    let tool = BrowserCtlTool::new();
    if request::needs_attach(&args.command) {
        call(&tool, request::attach(&args)).await?;
    }
    let result = call(&tool, request::command(&args)).await?;
    print_output(&result.output, args.json);
    Ok(())
}

async fn call(tool: &BrowserCtlTool, payload: Value) -> Result<codetether_tool_core::ToolResult> {
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
