//! Validate a follow-up and choose active steering or an idle turn.
use super::{args::Args, trigger};
use crate::{MessagingHost, Route};
use anyhow::{Result, bail};
use codetether_tool_core::ToolResult;
use serde_json::Value;

pub(super) async fn run<H: MessagingHost>(input: Value) -> Result<ToolResult> {
    let args: Args = serde_json::from_value(input)?;
    if args.message.trim().is_empty() {
        bail!("Empty message can't be sent to an agent");
    }
    if let Some(result) = crate::ensure::ready::<H>(&args.context, &args.target).await? {
        return Ok(result);
    }
    match H::steer(
        &args.target,
        args.context.session_id.as_deref(),
        &args.message,
    )
    .await
    {
        Route::Steered => Ok(ToolResult::success(String::new())),
        Route::NotFound => Ok(ToolResult::error(format!(
            "Agent {} not found",
            args.target
        ))),
        Route::Idle => trigger::run::<H>(args).await,
    }
}
