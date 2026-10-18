//! Restore and replace the active provider/model configuration between steps.
//!
//! [StepVars] borrows mutable loop fields. [restore_step_model] restores the
//! original model stored by the native session, while [apply_failover] selects
//! an alternate and prepares its context. [Host] supplies native services.
mod apply;
mod host;
mod original;
mod restore;
mod vars;
pub use apply::apply_failover;
pub use host::Host;
pub use restore::restore_step_model;
pub use vars::StepVars;

mod begin;
pub use begin::begin_step;
