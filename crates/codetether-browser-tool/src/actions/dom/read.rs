//! Browserctl DOM read action adapters.

use crate::helpers::optional_string;
use crate::input::BrowserCtlInput;
use codetether_browser::{BrowserCommand, BrowserError, BrowserOutput, request::ScopeRequest};

/// Build and execute a text extraction command.
///
/// # Errors
///
/// Returns [`BrowserError`] when command execution fails.
pub(crate) async fn text(input: &BrowserCtlInput) -> Result<BrowserOutput, BrowserError> {
    let request = ScopeRequest {
        selector: optional_string(&input.selector).map(str::to_string),
        frame_selector: optional_string(&input.frame_selector).map(str::to_string),
    };
    super::super::execute(input, BrowserCommand::Text(request)).await
}

/// Build and execute an HTML extraction command.
///
/// # Errors
///
/// Returns [`BrowserError`] when command execution fails.
pub(crate) async fn html(input: &BrowserCtlInput) -> Result<BrowserOutput, BrowserError> {
    let request = ScopeRequest {
        selector: optional_string(&input.selector).map(str::to_string),
        frame_selector: optional_string(&input.frame_selector).map(str::to_string),
    };
    super::super::execute(input, BrowserCommand::Html(request)).await
}
