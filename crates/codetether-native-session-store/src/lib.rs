//! Native session persistence and post-commit service integration.
//!
//! [Host] connects the shared store to recall, proactive preparation and archival.
//! [Session] exposes only archive configuration beyond existing store contracts.
//! Applications use [impl_session_lifecycle] to retain their Session methods.
pub mod archive;
mod bridge;
mod lifecycle;
mod session;
pub use codetether_session_store::save;
pub use session::Session;
use std::marker::PhantomData;
/// Persistence service host for the application session type S.
/// # Examples
/// ```
/// let _: Option<codetether_native_session_store::Host<()>> = None;
/// ```
pub struct Host<S>(PhantomData<fn() -> S>);
/// Restore a bounded transcript through the protected legacy migration path.
/// # Arguments
/// * path - Already resolved session locator.
/// * window - Maximum retained messages.
/// # Returns
/// Loaded session window, excluded message count and locator size.
/// # Errors
/// Propagates shared migration, decoding and storage failures.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_native_session_store::Session>(path: &std::path::Path) -> anyhow::Result<()> {
/// let loaded = codetether_native_session_store::load::<S>(path, 20).await?;
/// # Ok(()) }
/// ```
pub async fn load<S: Session>(
    path: &std::path::Path,
    window: usize,
) -> anyhow::Result<codetether_session_store::TailLoad<S>> {
    codetether_session_store::load::<S>(path, window, codetether_session_store::legacy::ensure::<S>)
        .await
}
/// Macro implementation types; applications should use the exported bridge.
#[doc(hidden)]
pub mod __private {
    pub use anyhow::Result;
    pub use codetether_session_store::{TailLoad, lifecycle::Host};
}
