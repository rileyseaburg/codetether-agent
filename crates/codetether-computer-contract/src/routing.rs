//! Input-mode routing: shadow requests must never fall through to real input.

use super::input::{ComputerUseAction as Action, ComputerUseInput, InputMode};

/// Decide whether the request uses shadow input.
///
/// # Arguments
/// * `input` - Typed action and input mode.
/// # Returns
/// True for shadow input actions; read-only capture and discovery stay native.
/// # Examples
/// ```
/// let input = serde_json::from_str(r#"{"action":"click","input_mode":"shadow"}"#).unwrap();
/// assert!(codetether_computer_contract::routing::shadow(&input));
/// ```
pub fn shadow(input: &ComputerUseInput) -> bool {
    input.input_mode == InputMode::Shadow
        && !matches!(
            input.action,
            Action::Snapshot
                | Action::WindowSnapshot
                | Action::Ocr
                | Action::OcrStatus
                | Action::ListApps
                | Action::RequestApp
                | Action::WaitMs
        )
}

/// Detect application selectors that the action cannot yet enforce.
///
/// # Arguments
/// * `input` - Request with optional application selectors.
/// # Returns
/// True when dispatch must reject the request before execution.
/// # Examples
/// ```
/// let input = serde_json::from_str(r#"{"action":"click","app":"editor"}"#).unwrap();
/// assert!(codetether_computer_contract::routing::app_gated(&input));
/// ```
pub fn app_gated(input: &ComputerUseInput) -> bool {
    !matches!(
        input.action,
        Action::Status | Action::ListApps | Action::RequestApp | Action::OcrStatus
    ) && (input.app.is_some() || input.window_title_contains.is_some())
}

#[cfg(test)]
#[path = "routing_tests.rs"]
mod tests;
