//! Outer interactive loop and recoverable event errors.
//!
//! Implement [Host] to retain application drawing, event sources and shutdown
//! resources, then call [run]. The driver owns the order and recovery policy.
mod binding;
mod host;
mod resilient;
mod run;
pub use host::{Cycle, Finish, Host, Prepare};
pub use run::run;
