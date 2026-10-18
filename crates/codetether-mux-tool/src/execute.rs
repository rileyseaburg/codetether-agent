//! Dispatch mux-only control actions.

use anyhow::Result;
use codetether_tool_core::ToolResult;

use super::args::Args;

pub(super) async fn run<H: crate::Host>(args: Args) -> Result<ToolResult> {
    match args.action.as_str() {
        "list" => super::operations::list().await,
        "read" => super::operations::read::<H>(super::operations::target(&args)?).await,
        "status" => super::operations::status(super::operations::target(&args)?).await,
        "steer" => {
            super::operations::steer(
                super::operations::target(&args)?,
                super::operations::message(&args)?,
            )
            .await
        }
        "interact" => super::operations::interact(super::operations::target(&args)?).await,
        "watch" => {
            super::operations::watch(super::operations::target(&args)?, args.timeout_ms).await
        }
        "start" => super::lifecycle::start(&args).await,
        "roll" => super::lifecycle::roll(&args).await,
        "stop" => super::lifecycle::stop(&args).await,
        action => Ok(ToolResult::error(format!("unknown mux action: {action}"))),
    }
}

/// Parse and execute one mux-manager tool request.
/// # Arguments
/// Receives the original JSON tool input.
/// # Returns
/// Structured observation output or lifecycle acceptance.
/// # Errors
/// Returns malformed input, missing arguments, lifecycle safety or control errors.
/// # Examples
/// ```rust,no_run
/// # async fn example<H: codetether_mux_tool::Host>() -> anyhow::Result<()> {
/// let _ = codetether_mux_tool::execute::<H>(serde_json::json!({"action":"list"})).await?;
/// # Ok(()) }
/// ```
pub async fn execute<H: crate::Host>(input: serde_json::Value) -> Result<ToolResult> {
    run::<H>(serde_json::from_value(input)?).await
}
