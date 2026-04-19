use super::super::helpers::require_string;
use super::super::input::BrowserCtlInput;
use crate::browser::{
    BrowserCommand,
    request::{ScreenshotRequest, WaitRequest},
};

pub(in crate::tool::browserctl) async fn wait(
    input: &BrowserCtlInput,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    let request = WaitRequest {
        text: input.text.clone(),
        text_gone: input.text_gone.clone(),
        url_contains: input.url_contains.clone(),
        selector: input.selector.clone(),
        frame_selector: input.frame_selector.clone(),
        state: input.state.clone().unwrap_or_else(|| "visible".into()),
        timeout_ms: input.timeout_ms.unwrap_or(30_000),
    };
    super::execute(input, BrowserCommand::Wait(request)).await
}

pub(in crate::tool::browserctl) async fn screenshot(
    input: &BrowserCtlInput,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    let _ = require_string(&input.path, "path")?;
    let request = ScreenshotRequest {
        selector: input.selector.clone(),
        frame_selector: input.frame_selector.clone(),
        full_page: input.full_page.unwrap_or(true),
    };
    super::execute(input, BrowserCommand::Screenshot(request)).await
}
