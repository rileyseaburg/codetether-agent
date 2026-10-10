//! Native monitor-picker list rendering.
use super::{state::State, text::wide};
use anyhow::{Result, ensure};
use codetether_companion_desktop::Monitor;
use windows_sys::Win32::{Foundation::HWND, UI::WindowsAndMessaging::*};

pub(super) fn populate(state: &State, monitors: &[Monitor]) -> Result<()> {
    let controls = state
        .controls
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Controls unavailable"))?;
    for monitor in monitors {
        let bounds = monitor.bounds();
        let primary = if monitor.is_primary() {
            " (primary)"
        } else {
            ""
        };
        append(
            controls.picker,
            &format!(
                "{} — {}×{}{primary}",
                monitor.device(),
                bounds.width(),
                bounds.height()
            ),
        )?;
    }
    Ok(())
}
pub(super) fn append(hwnd: HWND, label: &str) -> Result<()> {
    let label = wide(label);
    // SAFETY: the live string-only combo copies this terminated string.
    let result = unsafe { SendMessageW(hwnd, CB_ADDSTRING, 0, label.as_ptr() as isize) };
    ensure!(result >= 0, "Cannot populate monitor picker");
    Ok(())
}
