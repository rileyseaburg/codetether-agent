//! Select isolated Windows execution without recursing from the worker server.

use super::input::ComputerUseInput;

pub(super) async fn execute(input: ComputerUseInput) -> anyhow::Result<crate::tool::ToolResult> {
    #[cfg(windows)]
    {
        super::worker::execute(input).await
    }
    #[cfg(not(windows))]
    {
        super::platform::dispatch(&input).await
    }
}