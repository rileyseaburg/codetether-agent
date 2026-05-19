//! Browserctl tab action adapters.

use super::super::helpers::require_index;
use super::super::input::BrowserCtlInput;
use crate::browser::{
    BrowserCommand,
    request::{CloseTabRequest, NewTabRequest, TabSelectRequest},
};

/// Execute a tab list command.
///
/// # Errors
///
/// Returns [`crate::browser::BrowserError`] when execution fails.
pub(in crate::tool::browserctl) async fn tabs(
    input: &BrowserCtlInput,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    super::execute(input, BrowserCommand::Tabs).await
}

/// Build and execute a tab selection command.
///
/// # Errors
///
/// Returns [`crate::browser::BrowserError`] when `index` is missing or
/// execution fails.
pub(in crate::tool::browserctl) async fn tabs_select(
    input: &BrowserCtlInput,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
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
/// Returns [`crate::browser::BrowserError`] when execution fails.
pub(in crate::tool::browserctl) async fn tabs_new(
    input: &BrowserCtlInput,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
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
/// Returns [`crate::browser::BrowserError`] when execution fails.
pub(in crate::tool::browserctl) async fn tabs_close(
    input: &BrowserCtlInput,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    super::execute(
        input,
        BrowserCommand::TabsClose(CloseTabRequest { index: input.index }),
    )
    .await
}
