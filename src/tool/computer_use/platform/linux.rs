//! Linux desktop observation through session-compatible screenshot utilities.
//!
//! `dispatch` exposes status and whole-desktop capture, not input injection or
//! Windows HWND semantics. See `docs/linux-computer-use.md` for setup.

mod artifact;
mod backend;
mod command;
mod discovery;
mod image_file;
mod output;
mod request;
mod session;
mod snapshot;
mod status;
#[cfg(test)]
mod tests;

use crate::tool::computer_use::{
    input::ComputerUseAction as Action, input::ComputerUseInput, response,
};

pub(super) async fn dispatch(input: &ComputerUseInput) -> anyhow::Result<crate::tool::ToolResult> {
    let session = session::Session::detect();
    match input.action {
        Action::Status => Ok(status::result(session)),
        Action::Snapshot => request::snapshot(input, session).await,
        Action::ListApps
        | Action::RequestApp
        | Action::WindowSnapshot
        | Action::Ocr
        | Action::OcrStatus
        | Action::Click
        | Action::RightClick
        | Action::DoubleClick
        | Action::Drag
        | Action::MouseDown
        | Action::MouseMove
        | Action::MouseUp
        | Action::TypeText
        | Action::SetText
        | Action::ClickClient
        | Action::PressKey
        | Action::Scroll
        | Action::FocusViewport
        | Action::BlenderSelectFrame
        | Action::BringToFront
        | Action::WaitMs
        | Action::Stop => Ok(response::error_result(
            "Linux computer_use currently supports status and whole-desktop snapshot only; input, window targeting, and OCR are Windows-only",
        )),
    }
}
