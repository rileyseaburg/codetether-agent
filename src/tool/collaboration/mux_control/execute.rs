//! Dispatch mux-only control actions.

use crate::tool::ToolResult;
use anyhow::Result;

use super::args::Args;

pub(super) async fn run(args: Args) -> Result<ToolResult> {
    match args.action.as_str() {
        "list" => super::operations::list().await,
        "read" => super::operations::read(super::operations::target(&args)?).await,
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
