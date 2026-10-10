use super::{context::Context, dpi, ids, pending, reveal, safety, state::State, tray_events};
use anyhow::Result;
use windows_sys::Win32::{Foundation::*, UI::WindowsAndMessaging::*};

/// Runs `action` with the state, then drains pending safety/layout flags.
fn with_state(
    context: &Context,
    hwnd: HWND,
    action: impl FnOnce(&mut State) -> Result<()>,
) -> Result<Option<LRESULT>> {
    // A nested callback leaves the outer owner to drain the pending flags.
    let Ok(mut state) = context.state.try_borrow_mut() else {
        return Ok(Some(0));
    };
    action(&mut state)?;
    pending::apply(context, hwnd, &mut state)?;
    Ok(Some(0))
}

fn is_taskbar_restart(context: &Context, message: u32) -> bool {
    context
        .state
        .try_borrow()
        .is_ok_and(|s| s.taskbar_created != 0 && s.taskbar_created == message)
}

/// Maps window messages to handlers; `None` falls through to `DefWindowProcW`.
pub(super) fn handle(
    context: &Context,
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> Result<Option<LRESULT>> {
    safety::record(context, message, wparam);
    match message {
        WM_CLOSE => {
            reveal::hide(hwnd);
            Ok(Some(0))
        }
        WM_SYSCOMMAND if wparam & 0xfff0 == SC_MINIMIZE as usize => {
            reveal::hide(hwnd);
            Ok(Some(0))
        }
        WM_COMMAND => with_state(context, hwnd, |_| Ok(())),
        WM_TIMER if wparam == ids::PULSE => with_state(context, hwnd, |_| Ok(())),
        WM_WTSSESSION_CHANGE | WM_DISPLAYCHANGE | ids::RECHECK => {
            with_state(context, hwnd, |_| Ok(()))
        }
        WM_DPICHANGED => {
            // SAFETY: for WM_DPICHANGED, lparam points to the suggested RECT.
            dpi::resize(hwnd, Some(unsafe { &*(lparam as *const RECT) }))?;
            context.relayout.set(true);
            with_state(context, hwnd, |_| Ok(()))
        }
        ids::RELAYOUT => {
            context.relayout.set(true);
            with_state(context, hwnd, |_| Ok(()))
        }
        ids::TRAY_MESSAGE => {
            tray_events::handle(hwnd, lparam);
            Ok(Some(0))
        }
        m if is_taskbar_restart(context, m) => {
            with_state(context, hwnd, |s| tray_events::restore(hwnd, s))
        }
        _ => Ok(None),
    }
}
