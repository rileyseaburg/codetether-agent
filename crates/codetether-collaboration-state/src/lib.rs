//! Shared process-local collaboration status and parent wake-up state.
//!
//! [parent_activity] retains mailbox/input notifications until consumed.
//! [thread_status] publishes lifecycle changes over watch channels. Both the
//! agent runtime and its tools use these same registries and value types.
pub mod parent_activity;
pub mod thread_status;
pub use parent_activity::Activity;
pub use thread_status::ThreadStatus;
