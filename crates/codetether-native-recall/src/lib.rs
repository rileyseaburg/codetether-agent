//! Native recall services over persisted session evidence.
//!
//! [Host] combines canonical storage, indexed listing and worker cursors.
//! [Session] supplies the application's loaded transcript and bounded loader.
//! [build], [search], [prefetch] and [queue] expose the recall operations.
use std::marker::PhantomData;
pub mod build;
pub mod paths;
pub mod prefetch;
mod query;
pub mod queue;
pub mod search;
mod session;
pub mod session_io;
mod storage;
mod worker;
pub use session::Session;
/// Native recall host for session type S.
/// # Examples
/// `Host::<S>::default()` constructs the host without loading a session.
pub struct Host<S>(PhantomData<fn() -> S>);
impl<S> Default for Host<S> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
