//! Browserctl tab action adapters.

use super::super::helpers::require_index;
use super::super::input::BrowserCtlInput;
use codetether_browser::{
    BrowserCommand,
    request::{CloseTabRequest, NewTabRequest, TabSelectRequest},
};

/// Execute a tab list command.
///
/// # Errors
///
/// Returns [`codetether_browser::BrowserError`] when execution fails.
pub(crate) async fn tabs(
    input: &BrowserCtlInput,
) -> Result<codetether_browser::BrowserOutput, codetether_browser::BrowserError> {
    super::execute(input, BrowserCommand::Tabs).await
}

/// Build and execute a tab selection command.
///
/// # Errors
///
/// Returns [`codetether_browser::BrowserError`] when `index` is missing or
/// execution fails.
pub(crate) async fn tabs_select(
    input: &BrowserCtlInput,
) -> Result<codetether_browser::BrowserOutput, codetether_browser::BrowserError> {
    super::execute(
        input,
        BrowserCommand::TabsSelect(TabSelectRequest {
            index: require_index(input.index, "index")?,
        }),
    )
    .await
}

/// Build and execute a new-tab command.
///
/// # Errors
///
/// Returns [`codetether_browser::BrowserError`] when execution fails.
pub(crate) async fn tabs_new(
    input: &BrowserCtlInput,
) -> Result<codetether_browser::BrowserOutput, codetether_browser::BrowserError> {
    super::execute(
        input,
        BrowserCommand::TabsNew(NewTabRequest {
            url: input.url.clone().filter(|value| !value.trim().is_empty()),
        }),
    )
    .await
}

/// Build and execute a tab close command.
///
/// # Errors
///
/// Returns [`codetether_browser::BrowserError`] when execution fails.
pub(crate) async fn tabs_close(
    input: &BrowserCtlInput,
) -> Result<codetether_browser::BrowserOutput, codetether_browser::BrowserError> {
    super::execute(
        input,
        BrowserCommand::TabsClose(CloseTabRequest { index: input.index }),
    )
    .await
}
