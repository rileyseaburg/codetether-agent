//! Authorized tool execution with scoped progress and correlated audit records.
//!
//! Implement [Host] with application registry and policy services, then call
//! [execute]. Authorization runs before progress registration or tool execution.
//! Shared tool-reporting handles unknown tools and complete error cause chains.
mod execute;
mod heartbeat;
mod host;
mod run;
mod success;
pub use execute::execute;
pub use heartbeat::execute_with_heartbeat;
pub use host::Host;
