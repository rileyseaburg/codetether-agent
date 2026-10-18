//! Ordered execution of one normalized tool call.
//!
//! [run] sequences policy, execution and publication bindings. The shared driver
//! records approval declines in the current [crate::LoopState]; hosts retain
//! native session persistence and tool execution.
mod effects;
mod execute;
mod policy;
mod run;
pub use effects::Effects;
pub use execute::Execute;
pub use policy::Policy;
pub use run::run;
/// All native bindings needed by the single-call driver.
/// # Implementors
/// Implement Policy, Effects and Execute on the native prompt runner.
pub trait Host: Policy + Effects + Execute {}
impl<T: Policy + Effects + Execute> Host for T {}

/// Persistence, approval, and execution sequencing within one tool invocation.
pub mod invocation;
