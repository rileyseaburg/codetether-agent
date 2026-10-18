//! CLI objective display, confirmation and approved-run persistence.
//!
//! Prepare a [Prompt], create a run with [new_run], ask for confirmation with
//! [ask], then [persist] the approved objective and run.
mod ask;
mod create;
mod persist;
mod prompt;
mod render;
pub use ask::ask;
pub use create::new_run;
pub use persist::persist;
pub use prompt::Prompt;
