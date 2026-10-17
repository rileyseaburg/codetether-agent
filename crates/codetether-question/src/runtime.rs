//! Async rendezvous between an opted-in session and its question frontend.
//!
//! Wrap the foreground turn in [scope], call [ask], and have the frontend use
//! [pending], [act], and [dismiss] for that session. Spawned children do not
//! inherit the task-local capability. Dropping the waiting future clears state.
mod ask;
mod capability;
mod frontend;
mod store;
pub use ask::ask;
pub use capability::{available, scope};
pub use frontend::{act, dismiss, pending};
