//! Pairing edit-control access stays on the window thread.
use super::{state::State, text::wide};
use windows_sys::Win32::{
    Foundation::HWND,
    UI::{Input::KeyboardAndMouse::EnableWindow, WindowsAndMessaging::*},
};
use zeroize::{Zeroize, Zeroizing};

pub(super) fn take_code(edit: HWND) -> Zeroizing<String> {
    let mut buffer = [0u16; 64];
    // SAFETY: edit is a live child control and buffer capacity is passed.
    let len = unsafe { GetWindowTextW(edit, buffer.as_mut_ptr(), buffer.len() as i32) };
    let value = Zeroizing::new(String::from_utf16_lossy(&buffer[..len.max(0) as usize]));
    buffer.zeroize();
    // SAFETY: Windows copies the terminated empty string immediately.
    unsafe {
        SetWindowTextW(edit, wide("").as_ptr());
    }
    value
}
pub(super) fn busy(state: &State, busy: bool) {
    if let Some(c) = &state.controls {
        // SAFETY: both child HWNDs are live and owned by this thread.
        unsafe {
            EnableWindow(c.pair, i32::from(!busy));
            EnableWindow(c.code, i32::from(!busy));
        }
    }
}
