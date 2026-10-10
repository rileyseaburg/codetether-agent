use crate::TypingError;
use windows_sys::Win32::{
    Foundation::HWND, System::Threading::GetCurrentProcessId, UI::WindowsAndMessaging::*,
};

pub(super) fn current() -> Result<(HWND, HWND), TypingError> {
    // SAFETY: queries only; no focus stealing or window activation.
    unsafe {
        let foreground = GetForegroundWindow();
        if foreground.is_null() || IsWindowVisible(foreground) == 0 || IsIconic(foreground) != 0 {
            return Err(TypingError::TargetChanged);
        }
        let mut pid = 0;
        let thread = GetWindowThreadProcessId(foreground, &mut pid);
        if thread == 0 || pid == GetCurrentProcessId() {
            return Err(TypingError::TargetChanged);
        }
        let mut info: GUITHREADINFO = std::mem::zeroed();
        info.cbSize = std::mem::size_of::<GUITHREADINFO>() as u32;
        if GetGUIThreadInfo(thread, &mut info) == 0
            || info.hwndFocus.is_null()
            || (info.hwndFocus != foreground && IsChild(foreground, info.hwndFocus) == 0)
            || IsWindowVisible(info.hwndFocus) == 0
            || info.flags
                & (GUI_INMENUMODE | GUI_INMOVESIZE | GUI_POPUPMENUMODE | GUI_SYSTEMMENUMODE)
                != 0
        {
            return Err(TypingError::TargetChanged);
        }
        Ok((foreground, info.hwndFocus))
    }
}
