//! Native proactive context preparation backed by the session store.
//!
//! [prepare] and [schedule] launch the shared proactive worker using [Host].
//! [store] owns validated prepared-index sidecar paths. Applications implement
//! [Session] for their existing message window and bounded tail loader.
mod cursor;
mod lifecycle;
mod session;
pub mod store;
mod worker;
pub use lifecycle::{prepare, prepared_index, register, remove, schedule};
pub use session::Session;
use std::marker::PhantomData;
/// Native proactive worker and projection host for session type S.
/// # Examples
/// ```
/// let host = codetether_native_proactive::Host::<()>::default();
/// drop(host);
/// ```
pub struct Host<S>(PhantomData<fn() -> S>);
impl<S> Default for Host<S> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

/// Native background-model policy and typed notifications.
mod background;
pub use background::{Background, Notify};
/// Native RLM summary-node production.
pub mod summary;
