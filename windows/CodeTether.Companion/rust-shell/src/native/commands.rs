use super::{background, ids, pairing, picker, reveal, state::State};
use windows_sys::Win32::{Foundation::HWND, UI::WindowsAndMessaging::*};

pub(super) fn handle(hwnd: HWND, state: &mut State, word: usize) {
    let command = (word & 0xffff) as u16;
    let notification = ((word >> 16) & 0xffff) as u32;
    if notification == BN_CLICKED || command == ids::OPEN {
        state.pair_error.clear();
    }
    if command != ids::PICKER && command != ids::OPEN && notification != BN_CLICKED {
        return;
    }
    match command {
        ids::OPEN => reveal::show(hwnd),
        ids::BACKGROUND if state.device.is_some() => reveal::hide(hwnd),
        ids::PICKER if notification == CBN_SELCHANGE => picker::choose(state),
        ids::REFRESH => picker::refresh(state),
        ids::PAIR => pairing::begin(state),
        ids::START => state.resume(),
        ids::PAUSE => state.pause(),
        ids::STOP => {
            background::forget(state);
            picker::clear(state);
            state.status =
                "Capture OFF — unpaired; device credential forgotten and selection cleared.";
        }
        ids::EXIT => {
            state.shutdown();
            // SAFETY: the current thread owns the message loop.
            unsafe {
                PostQuitMessage(0);
            }
        }
        _ => {}
    }
}
