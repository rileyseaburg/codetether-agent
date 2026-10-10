use super::{ids::CLASS, procedure::procedure, text::failure};
use anyhow::Result;
use windows_sys::Win32::{
    Foundation::HINSTANCE, Graphics::Gdi::*, System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::*,
};

pub(super) struct Class(pub(super) HINSTANCE);

impl Class {
    pub(super) fn register() -> Result<Self> {
        // SAFETY: null asks for the module containing this process's entry point.
        let module = unsafe { GetModuleHandleW(std::ptr::null()) };
        if module.is_null() {
            return Err(failure("Cannot locate companion module"));
        }
        // SAFETY: zero initializes optional class fields; handles below are borrowed.
        let mut class: WNDCLASSW = unsafe { std::mem::zeroed() };
        class.hInstance = module;
        class.lpszClassName = CLASS;
        class.lpfnWndProc = Some(procedure);
        class.hbrBackground = (COLOR_WINDOW + 1) as usize as HBRUSH;
        // SAFETY: the system owns these shared resources for the process lifetime.
        class.hCursor = unsafe { LoadCursorW(std::ptr::null_mut(), IDC_ARROW) };
        class.hIcon = unsafe { LoadIconW(std::ptr::null_mut(), IDI_APPLICATION) };
        if class.hCursor.is_null() || class.hIcon.is_null() {
            return Err(failure("Cannot load companion window resources"));
        }
        // SAFETY: the class name is static and the callback has the system ABI.
        if unsafe { RegisterClassW(&class) } == 0 {
            return Err(failure("Cannot register companion window"));
        }
        Ok(Self(module))
    }
}

impl Drop for Class {
    fn drop(&mut self) {
        // SAFETY: the window guard is dropped before its registered class.
        unsafe {
            UnregisterClassW(CLASS, self.0);
        }
    }
}
