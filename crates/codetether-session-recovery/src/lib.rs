//! Identity-preserving durable session recovery and optional prompt execution.
//!
//! Implement RecoverySession for application storage, then call resume to
//! verify workspace ownership before applying metadata or executing a prompt.
mod execution;
mod host;
mod request;
mod service;
pub use host::RecoverySession;
pub use request::ResumeRequest;
pub use service::resume;
