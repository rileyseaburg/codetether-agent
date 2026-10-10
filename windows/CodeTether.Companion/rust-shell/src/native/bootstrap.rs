use super::{
    controls::Controls, dpi, layout, picker, session::SessionWatch, state::State, text::failure,
    timer::Timer, tray::Tray,
};
use anyhow::Result;
use windows_sys::Win32::{Foundation::HWND, UI::WindowsAndMessaging::RegisterWindowMessageW};

pub(super) fn initialize(hwnd: HWND, state: &mut State) -> Result<()> {
    // SAFETY: the static string requests Explorer's documented restart notification.
    state.taskbar_created = unsafe { RegisterWindowMessageW(windows_sys::w!("TaskbarCreated")) };
    if state.taskbar_created == 0 {
        return Err(failure("Cannot watch the system tray"));
    }
    state.controls = Some(Controls::create(hwnd)?);
    dpi::resize(hwnd, None)?;
    layout::update(hwnd, state)?;
    state.resources.session = Some(SessionWatch::register(hwnd)?);
    state.resources.timer = Some(Timer::start(hwnd)?);
    state.resources.tray = Some(Tray::add(hwnd)?);
    picker::refresh(state);
    if let Some(controls) = &state.controls {
        controls.set_status(&state.display());
    }
    Ok(())
}
