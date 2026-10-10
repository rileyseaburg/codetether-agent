use super::{
    class::Class, context::Context, instance::Instance, messages, text::notice, window::Window,
};
use anyhow::{Result, ensure};
use windows_sys::Win32::UI::HiDpi::{
    AreDpiAwarenessContextsEqual, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
    GetThreadDpiAwarenessContext, SetProcessDpiAwarenessContext,
};

pub(crate) fn run() -> Result<()> {
    let Some(_instance) = Instance::acquire()? else {
        notice("Screen Companion is already running. Open its controls from the system tray.");
        return Ok(());
    };
    // SAFETY: set awareness before creating any window. A manifest may set it first.
    let configured =
        unsafe { SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) };
    // SAFETY: these functions inspect the current UI thread only.
    ensure!(
        configured != 0
            || unsafe {
                AreDpiAwarenessContextsEqual(
                    GetThreadDpiAwarenessContext(),
                    DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
                )
            } != 0,
        "Per-monitor-v2 DPI awareness is required"
    );
    let class = Class::register()?;
    // The window borrows this address-stable context through native destruction.
    let context = Box::new(Context::new());
    let window = Window::create(&class, &context)?;
    let result = messages::pump(window.0);
    drop(window);
    if let Some(error) = context.failure.borrow_mut().take() {
        return Err(error);
    }
    result
}
