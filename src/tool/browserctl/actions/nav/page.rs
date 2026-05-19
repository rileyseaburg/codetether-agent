//! Browserctl page navigation action adapters.

use crate::browser::{BrowserCommand, BrowserError, BrowserOutput, request::NavigationRequest};
use crate::tool::browserctl::helpers::require_string;
use crate::tool::browserctl::input::BrowserCtlInput;

/// Build and execute a goto command.
///
/// # Errors
///
/// Returns [`BrowserError`] when `url` is missing or execution fails.
pub(in crate::tool::browserctl) async fn goto(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    let request = NavigationRequest {
        url: require_string(&input.url, "url")?.to_string(),
        wait_until: input
            .wait_until
            .clone()
            .unwrap_or_else(|| "domcontentloaded".into()),
    };
    super::super::execute(input, BrowserCommand::Goto(request)).await
}

/// Execute a back command.
///
/// # Errors
///
/// Returns [`BrowserError`] when execution fails.
pub(in crate::tool::browserctl) async fn back(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    super::super::execute(input, BrowserCommand::Back).await
}

/// Execute a reload command.
///
/// # Errors
///
/// Returns [`BrowserError`] when execution fails.
pub(in crate::tool::browserctl) async fn reload(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    super::super::execute(input, BrowserCommand::Reload).await
}
