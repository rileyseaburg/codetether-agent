//! Staged mouse primitive handlers.

use crate::platform::windows::computer_use::{
    hold_modifiers, modifier_vks, mouse_down, mouse_up, move_cursor, release_modifiers,
};
use crate::tool::computer_use::input::ComputerUseInput;

use super::mouse_result::staged_result;

pub async fn handle_mouse_down(
    input: &ComputerUseInput,
) -> anyhow::Result<crate::tool::ToolResult> {
    let (x, y) = super::validate::coords(input)?;
    let modifiers = modifier_vks(&input.modifiers)?;
    move_cursor(x, y)?;
    hold_modifiers(&modifiers)?;
    let button = mouse_down(input.button.as_deref())?;
    Ok(staged_result("mouse_down", button, Some((x, y)), input))
}

pub async fn handle_mouse_move(
    input: &ComputerUseInput,
) -> anyhow::Result<crate::tool::ToolResult> {
    let (x, y) = super::validate::coords(input)?;
    move_cursor(x, y)?;
    Ok(staged_result("mouse_move", "", Some((x, y)), input))
}

pub async fn handle_mouse_up(input: &ComputerUseInput) -> anyhow::Result<crate::tool::ToolResult> {
    let target = target(input)?;
    if let Some((x, y)) = target {
        move_cursor(x, y)?;
    }
    let button = mouse_up(input.button.as_deref())?;
    let modifiers = modifier_vks(&input.modifiers)?;
    release_modifiers(&modifiers)?;
    Ok(staged_result("mouse_up", button, target, input))
}

fn target(input: &ComputerUseInput) -> anyhow::Result<Option<(i32, i32)>> {
    if input.x.is_some() || input.y.is_some() {
        return super::validate::coords(input).map(Some);
    }
    Ok(None)
}
