//! `codetether context` subcommand.

use anyhow::{Result, bail};
use serde_json::json;

use crate::cli::{
    ContextArgs, ContextBrowseArgs, ContextBrowseCommand, ContextCommand, ContextResetArgs,
};
use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;
use crate::tool::context_browse::ContextBrowseTool;
use crate::tool::context_reset::format_reset_marker;
use crate::tool::{Tool, ToolResult};

const DEFAULT_RESET_SUMMARY: &str = "Manual context reset requested from the CLI.";

pub async fn execute(args: ContextArgs) -> Result<()> {
    match args.command {
        ContextCommand::Reset(args) => reset(args).await,
        ContextCommand::Browse(args) => browse(args).await,
    }
}

async fn reset(args: ContextResetArgs) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let mut session = Session::last_for_directory(Some(cwd.as_path())).await?;
    let summary = args
        .summary
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or(DEFAULT_RESET_SUMMARY);
    let marker = format_reset_marker(summary, summary.len());
    session.add_message(Message {
        role: Role::Assistant,
        content: vec![ContentPart::Text {
            text: marker.clone(),
        }],
    });
    session.save().await?;
    println!("{marker}");
    Ok(())
}

async fn browse(args: ContextBrowseArgs) -> Result<()> {
    let tool = ContextBrowseTool;
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
