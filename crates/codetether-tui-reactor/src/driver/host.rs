//! Native drawing, event processing and shutdown operations.
use std::future::Future;
/// Work performed before selecting the next event.
/// # Implementors
/// Native adapters update view data, mux status and terminal rendering.
pub trait Prepare {
    /// Refresh synchronous state before each draw.
    fn before_draw(&mut self);
    /// Publish mux status before drawing.
    fn update_mux(&mut self) -> impl Future<Output = ()>;
    /// Draw the terminal when the rendering policy permits.
    /// # Errors
    /// Returns a fatal terminal draw error.
    fn draw(&mut self) -> anyhow::Result<()>;
}
/// One event-processing iteration and its recoverable-error state.
/// # Implementors
/// Native adapters borrow their event sources and invoke shared dispatch.
pub trait Cycle {
    /// Select and handle one event, reporting whether to quit.
    /// # Errors
    /// Returns recoverable iteration errors for logging by the driver.
    fn select(&mut self) -> impl Future<Output = anyhow::Result<bool>>;
    /// Request a fresh draw after an iteration error.
    fn mark_redraw(&mut self);
}
/// Cleanup after the loop exits normally.
/// # Implementors
/// Native adapters consume their owned bridge and mux reporter during shutdown.
pub trait Finish: Sized {
    /// Finish mux, bridge and session shutdown in the existing order.
    fn finish(self) -> impl Future<Output = ()>;
}
/// Complete loop-driver binding.
/// # Implementors
/// Any type implementing [Prepare], [Cycle] and [Finish].
pub trait Host: Prepare + Cycle + Finish {}
impl<T: Prepare + Cycle + Finish> Host for T {}
