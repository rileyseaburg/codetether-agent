use super::{
    ids,
    text::{failure, wide},
};
use anyhow::Result;
use windows_sys::Win32::{
    Foundation::HWND,
    UI::{Shell::*, WindowsAndMessaging::*},
};

pub(super) struct Tray {
    pub(super) data: NOTIFYICONDATAW,
}

impl Tray {
    pub(super) fn add(hwnd: HWND) -> Result<Self> {
        // SAFETY: zero is the documented default for unused NOTIFYICONDATA fields.
        let mut data: NOTIFYICONDATAW = unsafe { std::mem::zeroed() };
        data.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
        data.hWnd = hwnd;
        data.uID = 1;
        data.uFlags = NIF_ICON | NIF_MESSAGE | NIF_TIP;
        data.uCallbackMessage = ids::TRAY_MESSAGE;
        // SAFETY: IDI_APPLICATION is a shared system icon, not owned by this guard.
        data.hIcon = unsafe { LoadIconW(std::ptr::null_mut(), IDI_APPLICATION) };
        if data.hIcon.is_null() {
            return Err(failure("Cannot load tray icon"));
        }
        let tip = wide("CodeTether Screen Companion — capture OFF");
        data.szTip[..tip.len()].copy_from_slice(&tip);
        // SAFETY: structure size is set and the icon and window remain live.
        if unsafe { Shell_NotifyIconW(NIM_ADD, &data) } == 0 {
            return Err(failure("Cannot install the capture-OFF tray icon"));
        }
        Ok(Self { data })
    }
}

impl Drop for Tray {
    fn drop(&mut self) {
        // SAFETY: the matching NIM_ADD succeeded and this guard owns its tray ID.
        unsafe {
            Shell_NotifyIconW(NIM_DELETE, &self.data);
        }
    }
}
