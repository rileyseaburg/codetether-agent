//! Validate target selection and supported actions before posting.
mod bounds;
use super::super::input::{ComputerUseAction as A, ComputerUseInput};
use anyhow::{Result, ensure};

pub(super) fn request(input: &ComputerUseInput) -> Result<i64> {
    let hwnd = input.hwnd.unwrap_or(0);
    ensure!(
        hwnd > 0 && isize::try_from(hwnd).is_ok(),
        "Shadow requires an explicit positive HWND"
    );
    ensure!(
        input.modifiers.is_empty(),
        "Shadow rejects modifiers; global keyboard state is never synthesized"
    );
    ensure!(
        input.app.is_none()
            && input.window_title_contains.is_none()
            && input.viewport_child_hwnd.is_none(),
        "Shadow only targets explicit hwnd, not app/title/viewport selectors"
    );
    let supported = matches!(
        input.action,
        A::Status
            | A::Stop
            | A::Click
            | A::RightClick
            | A::DoubleClick
            | A::MouseMove
            | A::MouseDown
            | A::MouseUp
            | A::Drag
            | A::Scroll
            | A::TypeText
            | A::PressKey
    );
    ensure!(supported, "Unsupported shadow action; no physical fallback");
    bounds::check(input)?;
    if let Some(button) = input.button.as_deref() {
        super::button::parse(Some(button))?;
    }
    if matches!(input.action, A::TypeText | A::PressKey) {
        super::keyboard::validate(input)?;
    }
    Ok(hwnd)
}
