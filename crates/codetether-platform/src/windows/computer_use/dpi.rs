//! Scoped physical-pixel coordinates for native capture and shadow input.
//!
//! The previous context is restored before returning a pooled blocking thread.

use windows::Win32::UI::HiDpi::{
    DPI_AWARENESS_CONTEXT, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2, SetThreadDpiAwarenessContext,
};

/// Scoped per-monitor DPI awareness for operations using physical pixel coordinates.
///
/// Dropping the guard restores the current thread's previous awareness context.
///
/// # Examples
/// ```no_run
/// use codetether_platform::windows::computer_use::dpi::DpiContext;
/// let guard = DpiContext::enter()?;
/// drop(guard);
/// # Ok::<(), anyhow::Error>(())
/// ```
pub struct DpiContext(DPI_AWARENESS_CONTEXT);

impl DpiContext {
    /// Enter per-monitor V2 coordinates, or fail instead of misplacing input.
    /// # Returns
    /// A guard that restores the calling thread's previous DPI awareness on drop.
    ///
    /// # Errors
    /// Returns an error when Windows refuses the per-monitor awareness context.
    pub fn enter() -> anyhow::Result<Self> {
        let previous =
            unsafe { SetThreadDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) };
        anyhow::ensure!(
            !previous.0.is_null(),
            "Cannot establish physical-pixel coordinates"
        );
        Ok(Self(previous))
    }
}

impl Drop for DpiContext {
    fn drop(&mut self) {
        // No pointer/focus changes; restore only this thread's coordinate context.
        unsafe { SetThreadDpiAwarenessContext(self.0) };
    }
}
