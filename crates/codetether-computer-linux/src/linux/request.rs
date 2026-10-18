//! Reject Windows window selectors rather than silently capturing another target.

use super::{backend, session::Session, snapshot};
use codetether_computer_contract::{input::ComputerUseInput, response};

pub(super) async fn snapshot(
    input: &ComputerUseInput,
    session: Session,
) -> anyhow::Result<codetether_tool_core::ToolResult> {
    if input.hwnd.is_some()
        || input.app.is_some()
        || input.window_title_contains.is_some()
        || input.client_area
        || input.viewport_child_hwnd.is_some()
    {
        return Ok(response::error_result(
            "Linux snapshot captures the whole desktop; window/app selectors are unsupported",
        ));
    }
    match snapshot::capture(session, &backend::available(session)).await {
        Ok(result) => Ok(result),
        Err(error) => Ok(response::error_result(format!("{error:#}"))),
    }
}
