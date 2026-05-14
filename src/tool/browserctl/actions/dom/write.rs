//! Browserctl DOM write action adapters.

use crate::browser::{
    BrowserCommand, BrowserError, BrowserOutput,
    request::{FillRequest, KeyPressRequest, SelectorRequest, TypeRequest},
};
use crate::tool::browserctl::helpers::require_string;
use crate::tool::browserctl::input::BrowserCtlInput;

/// Build and execute a selector click command.
///
/// # Errors
///
/// Returns [`BrowserError`] when `selector` is missing or execution fails.
pub(in crate::tool::browserctl) async fn click(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    let request = SelectorRequest {
        selector: require_string(&input.selector, "selector")?.to_string(),
        frame_selector: input.frame_selector.clone(),
    };
    super::super::execute(input, BrowserCommand::Click(request)).await
}

/// Build and execute a fill command.
///
/// # Errors
///
/// Returns [`BrowserError`] when `selector` or `value` is missing, or execution
/// fails.
pub(in crate::tool::browserctl) async fn fill(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    let request = FillRequest {
        selector: require_string(&input.selector, "selector")?.to_string(),
        value: require_string(&input.value, "value")?.to_string(),
        frame_selector: input.frame_selector.clone(),
    };
    super::super::execute(input, BrowserCommand::Fill(request)).await
}

/// Build and execute a type command.
///
/// # Errors
///
/// Returns [`BrowserError`] when `text` is missing or execution fails.
pub(in crate::tool::browserctl) async fn type_text(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    let request = TypeRequest {
        selector: require_string(&input.selector, "selector")?.to_string(),
        text: require_string(&input.text, "text")?.to_string(),
        delay_ms: input.delay_ms.unwrap_or(0),
        frame_selector: input.frame_selector.clone(),
    };
    super::super::execute(input, BrowserCommand::Type(request)).await
}

/// Build and execute a key press command.
///
/// # Errors
///
/// Returns [`BrowserError`] when `key` is missing or execution fails.
pub(in crate::tool::browserctl) async fn press(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    let request = KeyPressRequest {
        selector: require_string(&input.selector, "selector")?.to_string(),
        key: require_string(&input.key, "key")?.to_string(),
        frame_selector: input.frame_selector.clone(),
    };
    super::super::execute(input, BrowserCommand::Press(request)).await
}
