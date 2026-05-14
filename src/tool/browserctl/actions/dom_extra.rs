//! Browserctl higher-level DOM action adapters.

use super::super::helpers::{optional_string, require_string};
use super::super::input::BrowserCtlInput;
use crate::browser::{
    BrowserCommand,
    request::{ClickTextRequest, FillRequest, ToggleRequest},
};

/// Build and execute a visible-text click command.
///
/// # Errors
///
/// Returns [`BrowserError`] when `text` is missing or execution fails.
pub(in crate::tool::browserctl) async fn click_text(
    input: &BrowserCtlInput,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    let request = ClickTextRequest {
        selector: optional_string(&input.selector).map(str::to_string),
        frame_selector: optional_string(&input.frame_selector).map(str::to_string),
        text: require_string(&input.text, "text")?.to_string(),
        timeout_ms: input.timeout_ms.unwrap_or(5_000),
        exact: input.exact.unwrap_or(true),
        index: input.index.unwrap_or(0),
    };
    super::execute(input, BrowserCommand::ClickText(request)).await
}

/// Build and execute a native fill command.
///
/// # Errors
///
/// Returns [`BrowserError`] when `selector` or `value` is missing, or execution
/// fails.
pub(in crate::tool::browserctl) async fn fill_native(
    input: &BrowserCtlInput,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    let request = FillRequest {
        selector: require_string(&input.selector, "selector")?.to_string(),
        value: require_string(&input.value, "value")?.to_string(),
        frame_selector: input.frame_selector.clone(),
    };
    super::execute(input, BrowserCommand::FillNative(request)).await
}

/// Build and execute a toggle command.
///
/// # Errors
///
/// Returns [`BrowserError`] when `selector` is missing or execution fails.
pub(in crate::tool::browserctl) async fn toggle(
    input: &BrowserCtlInput,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    let request = ToggleRequest {
        selector: require_string(&input.selector, "selector")?.to_string(),
        frame_selector: input.frame_selector.clone(),
        text: require_string(&input.text, "text")?.to_string(),
        timeout_ms: input.timeout_ms.unwrap_or(5_000),
    };
    super::execute(input, BrowserCommand::Toggle(request)).await
}
