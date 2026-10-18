//! Shared ownership and control for session executors and delegated work.
//!
//! [ControlHandle] shares cancellation, pause and input cursors. [RunGuard]
//! acquires a session executor and cleans up steering resources. [ExecutionScope]
//! permits an explicit same-session handoff through task-local control. Native
//! durable-goal checks are supplied as a [Gate] when ownership is acquired.
mod binding;
mod execution;
mod execution_drop;
mod execution_run;
mod execution_scope;
mod gate;
mod handle;
mod handle_new;
mod handle_ops;
mod handle_ready;
mod handle_scope;
mod handle_steering;
mod plane;
pub use execution::RunGuard;
pub use execution_scope::ExecutionScope;
pub use gate::{Gate, GateFuture};
pub use handle::ControlHandle;
pub use plane::CONTROL;

mod active;
pub use active::ActiveCancel;

/// Optional control and per-delegate steering consumption.
pub mod delegated;
