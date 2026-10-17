//! Bubblewrap arguments and ordered filesystem mount planning.
//!
//! [build] renders a command invocation from sandbox policy and an optional
//! inherited seccomp descriptor. Runner selection and process spawning are external.
use codetether_sandbox_policy::SandboxPolicy;
mod args;
mod paths;
mod push;
pub use args::build;
