//! Browserctl motion action adapters.

use crate::browser::{
    BrowserCommand, BrowserError, BrowserOutput,
    request::{ScrollRequest, SelectorRequest},
};
use crate::tool::browserctl::helpers::{optional_string, require_string};
use crate::tool::browserctl::input::BrowserCtlInput;

/// Build and execute a hover command.
///
/// # Errors
///
/// Returns [`BrowserError`] when `selector` is missing or execution fails.
pub(in crate::tool::browserctl) async fn hover(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    super::super::execute(input, BrowserCommand::Hover(selector(input)?)).await
}

/// Build and execute a focus command.
///
/// # Errors
///
/// Returns [`BrowserError`] when `selector` is missing or execution fails.
pub(in crate::tool::browserctl) async fn focus(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    super::super::execute(input, BrowserCommand::Focus(selector(input)?)).await
}

/// Build and execute a blur command.
///
/// # Errors
///
/// Returns [`BrowserError`] when `selector` is missing or execution fails.
pub(in crate::tool::browserctl) async fn blur(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    super::super::execute(input, BrowserCommand::Blur(selector(input)?)).await
}

/// Build and execute a scroll command.
///
/// # Errors
///
/// Returns [`BrowserError`] when no scroll target or delta is supplied, or
/// execution fails.
pub(in crate::tool::browserctl) async fn scroll(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    let request = ScrollRequest {
        selector: optional_string(&input.selector).map(str::to_string),
        x: input.x,
        y: input.y,
        frame_selector: input.frame_selector.clone(),
    };
    if request.selector.is_none() && request.x.is_none() && request.y.is_none() {
        return Err(BrowserError::OperationFailed(
            "scroll requires selector, x, or y".into(),
        ));
    }
    super::super::execute(input, BrowserCommand::Scroll(request)).await
}

fn selector(input: &BrowserCtlInput) -> Result<SelectorRequest, BrowserError> {
    Ok(SelectorRequest {
        selector: require_string(&input.selector, "selector")?.to_string(),
        frame_selector: input.frame_selector.clone(),
    })
}
