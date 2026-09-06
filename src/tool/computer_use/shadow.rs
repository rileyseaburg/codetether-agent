//! HWND-targeted background input with no physical fallback.
//!
//! `client_area=false` means outer-window-relative coordinates; `true` means
//! client-relative. Mouse LPARAMs use client coordinates; wheel LPARAMs use
//! screen coordinates. Posting uses a scoped per-monitor DPI context for
//! physical pixels. Missing coordinates reuse only logical state.
//! Queue acceptance does not confirm application effects. UIPI, raw-input apps
//! and focus-dependent controls can reject or ignore these messages.

mod button;
mod coordinates;
mod drag;
mod keyboard;
mod keycodes;
mod mouse;
#[cfg(windows)]
mod native;
mod plan;
mod replay;
mod report;
#[cfg(windows)]
mod runtime;
mod status;
mod stop;
#[cfg(test)]
mod tests;
mod types;
mod validate;

use super::input::ComputerUseInput;
use crate::tool::ToolResult;

/// Queue HWND-specific input; application effects remain unverified.
/// Invalid requests and posting failures return unsuccessful tool results.
///
/// # Arguments
/// * `input` - An explicit HWND and one supported shadow action.
/// # Returns
/// A queue-only outcome with logical state and unverified application effects.
/// # Errors
/// Returns an error if the Windows blocking task cannot be joined.
/// # Examples
/// ```text
/// {"action":"click","input_mode":"shadow","hwnd":123,"client_area":true,"x":10,"y":20}
/// ```
pub(crate) async fn dispatch(input: &ComputerUseInput) -> anyhow::Result<ToolResult> {
    if let Err(error) = validate::request(input) {
        return Ok(report::failure(error));
    }
    #[cfg(windows)]
    {
        let input = input.clone();
        Ok(tokio::task::spawn_blocking(move || runtime::dispatch(&input)).await?)
    }
    #[cfg(not(windows))]
    Ok(report::failure(anyhow::anyhow!(
        "Shadow posting requires Windows; no physical fallback"
    )))
}