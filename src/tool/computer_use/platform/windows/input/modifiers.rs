//! Modifier handling for mouse actions.

use crate::platform::windows::computer_use::{hold_modifiers, modifier_vks, release_modifiers};

pub fn with_modifiers(
    modifiers: &[String],
    action: impl FnOnce() -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    let vks = modifier_vks(modifiers)?;
    hold_modifiers(&vks)?;
    let result = action();
    let release = release_modifiers(&vks);
    result.and(release)
}
