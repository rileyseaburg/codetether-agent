//! Platform-specific computer use implementations.

use crate::tool::computer_use::{input::ComputerUseInput, response};

#[cfg(target_os = "windows")]
mod windows;

pub async fn dispatch(input: &ComputerUseInput) -> anyhow::Result<crate::tool::ToolResult> {
    #[cfg(target_os = "windows")]
    {
        return windows::dispatch(input).await;
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = input;
        Ok(response::unsupported_platform_result())
    }
}
