//! Owned local swarm jobs and their bounded execution lifecycle.
//!
//! Build a [Job] from the stage's selected provider/workspace and call [execute]
//! with a native [Host]. This crate owns runtime scoping, concurrency permits,
//! retry policy, prompt assembly, outcome conversion and lifecycle events.
mod finish_events;
mod host;
mod job;
mod job_execute;
mod job_run;
mod prompts;
mod result;
mod retry;
mod start_events;
pub use host::Host;
pub use job::Job;
pub use job_run::execute;
