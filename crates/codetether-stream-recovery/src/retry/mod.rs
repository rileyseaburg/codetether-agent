//! Pending reconnect admission, backoff and request submission.
//!
//! Implement [Host] to bind session ownership and UI state, then call [execute]
//! with the available provider. Unavailable prerequisites retain the prompt.
mod host;
mod run;
pub use host::{Host, State};
pub use run::execute;
