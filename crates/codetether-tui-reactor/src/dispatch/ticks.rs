//! Periodic UI, audit and watchdog actions.
use std::future::Future;
/// Periodic work selected by reactor timers.
/// # Implementors
/// Native adapters supply watchdog checks, audit refresh and UI ticks.
pub trait Ticks {
    /// Check whether a stalled request needs restarting.
    fn watchdog(&mut self) -> impl Future<Output = ()>;
    /// Refresh audit data.
    fn audit(&mut self) -> impl Future<Output = ()>;
    /// Apply general UI tick updates.
    fn tick(&mut self) -> impl Future<Output = ()>;
    /// Check retries after the UI tick.
    fn tick_retry(&mut self) -> impl Future<Output = ()>;
}
