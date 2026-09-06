//! Pure logical status: no cursor, foreground, geometry, or input API calls.
use super::super::input::{ComputerUseAction, ComputerUseInput};
use super::{replay::Outcome, report, types::State};
use crate::tool::ToolResult;

pub(super) fn reply(input: &ComputerUseInput, state: State) -> Option<ToolResult> {
    matches!(input.action, ComputerUseAction::Status).then(|| {
        report::result(
            input.hwnd,
            state,
            Outcome::default(),
            serde_json::Value::Null,
        )
    })
}
