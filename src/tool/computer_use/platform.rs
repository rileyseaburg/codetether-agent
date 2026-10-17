//! Platform-specific computer use implementations.

use crate::tool::computer_use::input::ComputerUseInput;

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
use crate::tool::computer_use::response;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

pub async fn dispatch(input: &ComputerUseInput) -> anyhow::Result<crate::tool::ToolResult> {
    #[cfg(target_os = "windows")]
    {
        return windows::dispatch(input).await;
    }
    #[cfg(target_os = "linux")]
    {
        return linux::dispatch(input).await;
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        let _ = input;
        Ok(response::unsupported_platform_result())
    }
}
