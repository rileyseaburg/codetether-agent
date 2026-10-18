//! Platform-specific computer use implementations.

use crate::input::ComputerUseInput;

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
use crate::response;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

/// Dispatch directly to the current platform without entering the worker client.
///
/// # Arguments
/// * `input` - Typed computer-use request.
/// # Returns
/// Platform output or an unsupported-platform failure.
/// # Errors
/// Propagates the selected backend error.
/// # Examples
/// ```text
/// platform::dispatch(&input).await?
/// ```
pub async fn dispatch(
    input: &ComputerUseInput,
) -> anyhow::Result<codetether_tool_core::ToolResult> {
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
