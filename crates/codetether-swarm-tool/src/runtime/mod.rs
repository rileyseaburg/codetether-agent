//! Direct swarm scheduling and worker result collection.
//!
//! Implement [Host] and [Observer] for native provider, tool and UI bindings,
//! then call [execute]. The runtime owns concurrency, cancellation guards,
//! worktree integration, and aggregate reporting.
mod collect;
mod dispatch;
mod execute;
mod host;
mod inputs;
mod schedule;
mod worker;
pub use execute::execute;
pub use host::{Host, Observer};
pub use inputs::{LoopInput, SelectedProvider, WorkerContext};

mod context;
mod invocation;
mod messages;
mod outcome;
mod pending;
