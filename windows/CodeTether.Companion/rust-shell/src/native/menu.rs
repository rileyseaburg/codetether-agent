use super::{ids, text::wide};
use windows_sys::Win32::{Foundation::HWND, Foundation::POINT, UI::WindowsAndMessaging::*};

const ITEMS: [(u16, &str); 4] = [
    (ids::OPEN, "Open controls"),
    (ids::PAUSE, "Pause"),
    (ids::STOP, "Stop / unpair"),
    (ids::EXIT, "Exit"),
];

/// Shows the tray menu; selections arrive as ordinary `WM_COMMAND` messages.
pub(super) fn show(hwnd: HWND) {
    // SAFETY: the menu is created, used and destroyed here on the UI thread;
    // AppendMenuW copies each terminated label during the call.
    unsafe {
        let menu = CreatePopupMenu();
        if menu.is_null() {
            return;
        }
        for (id, label) in ITEMS {
            AppendMenuW(menu, MF_STRING, usize::from(id), wide(label).as_ptr());
        }
        let mut at = POINT { x: 0, y: 0 };
        GetCursorPos(&mut at);
        SetForegroundWindow(hwnd);
        TrackPopupMenu(menu, TPM_RIGHTBUTTON, at.x, at.y, 0, hwnd, std::ptr::null());
        PostMessageW(hwnd, WM_NULL, 0, 0);
        DestroyMenu(menu);
    }
}
