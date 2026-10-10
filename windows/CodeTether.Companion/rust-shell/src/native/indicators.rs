//! Local status and Resume control; pairing/selection replace the old Start gate.
use super::{ids, state::State, text::wide};
use anyhow::Result;
use windows_sys::Win32::{
    Foundation::HWND,
    UI::{Input::KeyboardAndMouse::EnableWindow, WindowsAndMessaging::*},
};

pub(super) fn update(hwnd: HWND, state: &mut State) -> Result<()> {
    let active = !state.background.paused
        && state.desktop_available
        && state
            .background
            .worker
            .as_ref()
            .is_some_and(|job| !job.stop.token.is_cancelled());
    if let Some(tray) = &mut state.resources.tray {
        tray.sharing(active)?;
    }
    if let Some(controls) = &state.controls {
        controls.set_status(&state.display());
    }
    let title = wide(if active {
        "CodeTether Screen Companion — sharing ON"
    } else {
        "CodeTether Screen Companion — sharing OFF / suspended"
    });
    let resume = state.background.paused && state.device.is_some() && state.selected.is_some();
    // SAFETY: main window and Resume child are live on this UI thread.
    unsafe {
        SetWindowTextW(hwnd, title.as_ptr());
        EnableWindow(GetDlgItem(hwnd, i32::from(ids::START)), i32::from(resume));
        EnableWindow(
            GetDlgItem(hwnd, i32::from(ids::BACKGROUND)),
            i32::from(state.device.is_some()),
        );
    }
    Ok(())
}
