//! Serialize shadow plans; persist only accepted per-HWND logical transitions.
mod store;
use super::super::input::{ComputerUseAction as A, ComputerUseInput};
use super::{
    native::{self, Target},
    plan, replay, report,
    types::State,
    validate,
};
use crate::tool::ToolResult;
use anyhow::{Result, ensure};
use serde_json::json;

pub(super) fn dispatch(input: &ComputerUseInput) -> ToolResult {
    execute(input).unwrap_or_else(report::failure)
}
fn execute(input: &ComputerUseInput) -> Result<ToolResult> {
    let hwnd = validate::request(input)?;
    let target = Target::open(hwnd)?;
    ensure!(
        !matches!(input.action, A::TypeText) || target.is_unicode(),
        "Shadow type_text requires a Unicode target HWND"
    );
    let mut states = store::lock()?;
    let entry = store::entry(&mut states, target)?;
    if let Some(status) = super::status::reply(input, entry.state) {
        return Ok(status);
    }
    let _dpi = crate::platform::windows::computer_use::dpi::DpiContext::enter()?;
    let plan = plan::build(input, target.geometry()?, entry.state)?;
    let before = native::observe();
    let outcome = replay::execute(&plan, &mut entry.state, |event| target.post(event));
    let stopped = matches!(input.action, A::Stop) && outcome.error.is_none();
    if stopped {
        entry.state = State::default();
    }
    let state = entry.state;
    if stopped {
        states.remove(&hwnd);
    }
    Ok(report::result(
        Some(hwnd),
        state,
        outcome,
        json!({
            "before": before, "after": native::observe(),
            "logical_state_cleared": stopped,
            "stop_scope": "Only this HWND; serialized after already running shadow actions"
        }),
    ))
}