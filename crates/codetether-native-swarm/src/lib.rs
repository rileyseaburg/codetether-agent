//! Native swarm services over the shared agent and execution runtimes.
//!
//! [AgentHost] owns registry lookup, runtime authorization and goal accounting.
//! [ExecutorHost] supplies native providers and local-stage execution, while
//! [workspace] binds rooted tools and [policy] composes autonomous instructions.
mod agent;
pub mod delegation;
mod executor;
pub mod policy;
mod providers;
pub mod workspace;
pub use agent::AgentHost;
pub use executor::ExecutorHost;
pub use providers::Providers;

#[cfg(test)]
mod tests;
