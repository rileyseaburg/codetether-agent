//! UTF-16 WM_CHAR and paired key messages without global key state.
use super::super::input::{ComputerUseAction as A, ComputerUseInput};
use super::{keycodes, types::Plan};
use anyhow::{Result, anyhow, ensure};

pub(super) fn validate(input: &ComputerUseInput) -> Result<()> {
    if matches!(input.action, A::TypeText) {
        let text = input
            .text
            .as_deref()
            .ok_or_else(|| anyhow!("text is required"))?;
        ensure!(
            text.encode_utf16().take(8193).count() <= 8192,
            "Text exceeds 8192 UTF-16 units"
        );
        ensure!(!text.contains('\0'), "Shadow text must not contain NUL");
    } else {
        let key = input
            .key
            .as_deref()
            .ok_or_else(|| anyhow!("key is required"))?;
        keycodes::lookup(key)?;
    }
    Ok(())
}
pub(super) fn append(input: &ComputerUseInput, plan: &mut Plan) -> Result<()> {
    if matches!(input.action, A::TypeText) {
        for unit in input.text.as_deref().unwrap_or_default().encode_utf16() {
            plan.push(0x0102, usize::from(unit), 1, 0);
        }
    } else {
        ensure!(
            plan.state.pending_key.is_none(),
            "Prior key-up failed; use stop first"
        );
        let (vk, scan, extended) = keycodes::lookup(input.key.as_deref().unwrap_or_default())?;
        plan.state.pending_key = Some((vk, keycodes::flags(scan, extended, true)));
        plan.push(
            0x0100,
            usize::from(vk),
            keycodes::flags(scan, extended, false),
            0,
        );
        release(plan);
    }
    Ok(())
}
pub(super) fn release(plan: &mut Plan) {
    if let Some((vk, flags)) = plan.state.pending_key.take() {
        plan.push(0x0101, usize::from(vk), flags, 0);
    }
}
