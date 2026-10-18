//! Sub-agent provider request construction and deadline outcomes.
//!
//! [build] copies the current conversation and advertised tools. [execute]
//! bounds the supplied completion future by the agent's remaining deadline.
mod build;
mod execute;
pub use build::build;
pub use execute::execute;
