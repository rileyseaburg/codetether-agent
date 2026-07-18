//! First-class spawn translation into the durable agent runtime.

use super::{args::Args, name, result};
use crate::tool::ToolResult;
use anyhow::Result;
use serde_json::json;

pub(super) async fn execute(args: Args) -> Result<ToolResult> {
    let fork_turns = args.resolved_fork_turns();
    let name = name::resolve(args.task_name)?;
    let output = super::super::legacy::execute(
        &args.context,
        json!({
            "action":"spawn", "name":name,
            "instructions":args.message, "detach":true,
            "fork_turns":fork_turns, "model":args.model
        })
        .as_object()
        .cloned()
        .expect("spawn payload is an object"),
    )
    .await?;
    Ok(result::attach_task_path(output, &args.context))
}
