//! Application-owned durable execution gate.
use anyhow::Result;
use std::{future::Future, pin::Pin};
/// Borrowing asynchronous result of an application's durable execution check.
/// # Examples
/// ```rust
/// fn check(_: &str) -> codetether_session_control::GateFuture<'_> {
///     Box::pin(async { Ok(()) })
/// }
/// ```
pub type GateFuture<'a> = Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>>;
/// Application check for journal errors, durable holds and goal budget gates.
/// The callback is retained by attached handles and inherited by child tasks.
/// # Examples
/// ```rust
/// use codetether_session_control::{Gate, GateFuture};
/// fn check(_: &str) -> GateFuture<'_> { Box::pin(async { Ok(()) }) }
/// let gate: Gate = check;
/// # let _ = gate;
/// ```
pub type Gate = for<'a> fn(&'a str) -> GateFuture<'a>;
/// Standalone handles have no durable session gate.
pub(crate) fn unrestricted(_: &str) -> GateFuture<'_> {
    Box::pin(async { Ok(()) })
}
