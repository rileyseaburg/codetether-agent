//! Resolve the durable session that issued a context-tool invocation.
//!
//! [load_calling_session] and [load_calling_window] prefer an injected session ID,
//! then the environment ID. Only callers without an ID use a directory lookup.
//! [Source] binds these decisions to the application's existing persistence API.
mod calling;
mod errors;
mod latest;
mod source;
pub use calling::{load_calling_session, load_calling_window};
pub use latest::load_latest_session;
pub use source::Source;
/// Injected field naming the durable session that issued the tool call.
pub const SESSION_ID_FIELD: &str = "__ct_session_id";
