//! Translate browse options into the shared history tool contract.
use crate::{ContextBrowseArgs, ContextBrowseCommand};
use anyhow::Result;
use codetether_context_browse::{BrowseSource, ContextBrowseTool};
use codetether_tool_core::{Tool, ToolResult};
use serde_json::json;

pub(crate) async fn browse<S: BrowseSource>(args: ContextBrowseArgs) -> Result<()> {
    let tool = ContextBrowseTool::<S>::new();
    let payload = match args.command {
        Some(ContextBrowseCommand::ShowTurn(args)) => {
            json!({"action": "show_turn", "turn": args.turn})
        }
        None | Some(ContextBrowseCommand::List) => json!({"action": "list"}),
    };
    print_result(tool.execute(payload).await?)
}

fn print_result(result: ToolResult) -> Result<()> {
    if result.success {
        println!("{}", result.output);
        Ok(())
    } else {
        bail!("{}", result.output)
    }
}
