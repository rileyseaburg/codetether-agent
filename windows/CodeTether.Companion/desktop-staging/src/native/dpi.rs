use crate::Error;
use std::{marker::PhantomData, rc::Rc};
use windows_sys::Win32::UI::HiDpi::*;

// Thread-local state must be restored on the same thread.
pub(super) struct Guard(DPI_AWARENESS_CONTEXT, PhantomData<Rc<()>>);
impl Guard {
    pub(super) fn enter() -> Result<Self, Error> {
        // SAFETY: changes only this thread; the returned context is restored.
        let previous =
            unsafe { SetThreadDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) };
        if previous.is_null() {
            Err(Error::Unavailable)
        } else {
            Ok(Self(previous, PhantomData))
        }
    }

    pub(super) fn restore(mut self) -> Result<(), Error> {
        // SAFETY: the saved context belongs to this thread; Guard is !Send.
        let restored = unsafe { SetThreadDpiAwarenessContext(self.0) };
        if restored.is_null() {
            return Err(Error::Unavailable);
        }
        self.0 = std::ptr::null_mut();
        Ok(())
    }
}
impl Drop for Guard {
    fn drop(&mut self) {
        if !self.0.is_null() {
            // SAFETY: retry restoring this thread's context on error paths.
            unsafe { SetThreadDpiAwarenessContext(self.0) };
        }
    }
}
