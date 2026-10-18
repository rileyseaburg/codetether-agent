//! Remote swarm worker process lifecycle.
//!
//! [run] loads a payload, selects its workspace, resolves application resources,
//! executes the task, and emits its result after stopping branch observations.
//! Implement [Host] to provide tool/provider construction and agent execution.
mod host;
mod resources;
mod runtime;
mod setup;

pub use host::Host;
pub use resources::Resources;
pub use runtime::run;
