//! Platform-specific computer use implementations.

use crate::tool::computer_use::{input::ComputerUseInput, response};

mod windows;

pub async fn dispatch(input: &ComputerUseInput) -> anyhow::Result<crate::tool::ToolResult> {
    if std::env::consts::OS == "windows" {
        return windows::dispatch(input).await;
    }
    Ok(response::unsupported_platform_result())
}
