//! Browserctl device input action adapters.

use super::super::helpers::{require_point, require_string};
use super::super::input::BrowserCtlInput;
use codetether_browser::{
    BrowserCommand,
    request::{KeyboardPressRequest, KeyboardTypeRequest, PointerClick},
};

/// Build and execute a coordinate click command.
///
/// # Errors
///
/// Returns [`BrowserError`] when coordinates are missing or execution fails.
pub(crate) async fn mouse_click(
    input: &BrowserCtlInput,
) -> Result<codetether_browser::BrowserOutput, codetether_browser::BrowserError> {
    let request = PointerClick {
        x: require_point(input.x, "x")?,
        y: require_point(input.y, "y")?,
    };
    super::execute(input, BrowserCommand::MouseClick(request)).await
}

/// Build and execute a keyboard type command.
///
/// # Errors
///
/// Returns [`BrowserError`] when `text` is missing or execution fails.
pub(crate) async fn keyboard_type(
    input: &BrowserCtlInput,
) -> Result<codetether_browser::BrowserOutput, codetether_browser::BrowserError> {
    let request = KeyboardTypeRequest {
        text: require_string(&input.text, "text")?.to_string(),
    };
    super::execute(input, BrowserCommand::KeyboardType(request)).await
}

/// Build and execute a keyboard press command.
///
/// # Errors
///
/// Returns [`BrowserError`] when `key` is missing or execution fails.
pub(crate) async fn keyboard_press(
    input: &BrowserCtlInput,
) -> Result<codetether_browser::BrowserOutput, codetether_browser::BrowserError> {
    let request = KeyboardPressRequest {
        key: require_string(&input.key, "key")?.to_string(),
    };
    super::execute(input, BrowserCommand::KeyboardPress(request)).await
}
