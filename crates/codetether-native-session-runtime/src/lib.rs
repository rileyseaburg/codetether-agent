//! Native session execution ownership, continuation and writeback journals.
//!
//! [`RunGuard`] and [`execution_scope::ExecutionScope`] use the shared session
//! controller with durable goal gates. [`continuation`] and [`swarm_monitor`]
//! accept the existing transcript contract; applications retain concrete session
//! types. [`journal`] appends writeback events beside the canonical session path.
pub use codetether_session_control::ControlHandle;
mod control_gate;
mod execution;
pub use execution::RunGuard;
/// Durable goal checks at native session execution boundaries.
pub mod boundary;
/// Final-message and explicit-goal continuation using native journals.
pub mod continuation;
/// Reentrant session execution scopes using the native durable goal gate.
pub mod execution_scope;
/// Typed writeback-journal persistence beside native session records.
pub mod journal;
/// Delegated-work monitoring with the native goal journal.
pub mod swarm_monitor;
