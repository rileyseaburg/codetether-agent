use super::{menu, reveal, state::State, tray::Tray};
use anyhow::Result;
use windows_sys::Win32::{
    Foundation::{HWND, LPARAM},
    UI::WindowsAndMessaging::*,
};

/// Routes tray mouse notifications. Neither action touches capture state.
pub(super) fn handle(hwnd: HWND, lparam: LPARAM) {
    match (lparam & 0xffff) as u32 {
        WM_LBUTTONUP | WM_LBUTTONDBLCLK => reveal::show(hwnd),
        WM_RBUTTONUP | WM_CONTEXTMENU => menu::show(hwnd),
        _ => {}
    }
}

/// Reinstalls the capture-OFF icon after Explorer restarts.
pub(super) fn restore(hwnd: HWND, state: &mut State) -> Result<()> {
    state.resources.tray = None;
    state.resources.tray = Some(Tray::add(hwnd)?);
    Ok(())
}
