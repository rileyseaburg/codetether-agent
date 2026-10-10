use super::{
    bootstrap, class::Class, context::Context, ids::CLASS, pending, reveal, text::failure,
};
use anyhow::Result;
use windows_sys::Win32::{Foundation::HWND, UI::WindowsAndMessaging::*};

pub(super) struct Window<'a>(pub(super) HWND, &'a Context);

impl<'a> Window<'a> {
    pub(super) fn create(class: &Class, context: &'a Context) -> Result<Self> {
        // SAFETY: the guard borrows context until after native destruction.
        // Initial sizing happens while hidden, once the window's DPI is available.
        // A tool window never gets a taskbar button; tray access remains available.
        let hwnd = unsafe {
            CreateWindowExW(
                WS_EX_TOOLWINDOW,
                CLASS,
                windows_sys::w!("CodeTether Screen Companion — capture OFF"),
                WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                640,
                500,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                class.0,
                (context as *const Context).cast(),
            )
        };
        if hwnd.is_null() {
            return Err(failure("Cannot create companion window"));
        }
        let window = Self(hwnd, context);
        {
            let mut state = context.state.borrow_mut();
            bootstrap::initialize(hwnd, &mut state)?;
            pending::apply(context, hwnd, &mut state)?;
        }
        reveal::show(hwnd);
        Ok(window)
    }
}

impl Drop for Window<'_> {
    fn drop(&mut self) {
        if let Ok(mut state) = self.1.state.try_borrow_mut() {
            state.shutdown();
        }
        // SAFETY: observers stop first; context and its fonts outlive every
        // child HWND, and destruction occurs on the owning UI thread.
        unsafe {
            if IsWindow(self.0) != 0 {
                DestroyWindow(self.0);
            }
        }
    }
}
