//! Input-mode routing: shadow requests must never fall through to real input.

use super::input::{ComputerUseAction as Action, ComputerUseInput, InputMode};

pub(super) fn shadow(input: &ComputerUseInput) -> bool {
    input.input_mode == InputMode::Shadow
        && !matches!(
            input.action,
            Action::Snapshot | Action::WindowSnapshot | Action::Ocr | Action::OcrStatus
                | Action::ListApps | Action::RequestApp | Action::WaitMs
        )
}

pub(super) fn app_gated(input: &ComputerUseInput) -> bool {
    !matches!(input.action, Action::Status | Action::ListApps | Action::RequestApp | Action::OcrStatus)
        && (input.app.is_some() || input.window_title_contains.is_some())
}

#[cfg(test)]
#[path = "routing_tests.rs"]
mod tests;