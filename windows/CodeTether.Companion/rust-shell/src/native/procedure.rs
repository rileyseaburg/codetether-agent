use super::{callback, context::Context};
use windows_sys::Win32::{Foundation::*, UI::WindowsAndMessaging::*};

pub(super) unsafe extern "system" fn procedure(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if message == WM_NCCREATE {
        // SAFETY: Windows supplies a valid CREATESTRUCTW for WM_NCCREATE.
        let create = unsafe { &*(lparam as *const CREATESTRUCTW) };
        // SAFETY: Window::create supplies a Context that outlives this HWND.
        unsafe {
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, create.lpCreateParams as isize);
        }
    }
    // SAFETY: only this procedure writes GWLP_USERDATA, using that Context.
    let pointer = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) } as *const Context;
    if message == WM_NCDESTROY {
        // SAFETY: clear the borrowed pointer before native teardown finishes.
        unsafe {
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
        }
    } else if !pointer.is_null() {
        // SAFETY: application ownership outlives every callback for this window.
        let context = unsafe { &*pointer };
        if let Some(result) = callback::handle(context, hwnd, message, wparam, lparam) {
            return result;
        }
    }
    // SAFETY: unhandled native messages are forwarded unchanged.
    unsafe { DefWindowProcW(hwnd, message, wparam, lparam) }
}
