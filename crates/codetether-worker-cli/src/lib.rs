//! Worker CLI credential fallback and optional HTTP-server lifecycle.
//!
//! Implement [Host] for the native worker entry points and call [execute].
//! Saved credentials are scoped to the requested server. HTTP-enabled workers
//! share one server state, and the HTTP task is aborted when the worker returns.
mod credentials;
mod host;
mod runtime;
pub use host::Host;
pub use runtime::execute;
