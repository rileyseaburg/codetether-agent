//! Execution evidence assembly independent of engine and command dispatch.
//!
//! Use [changed_files] for snapshot comparison, then [with_quality] to combine
//! the host engine summary and quality result into [ExecutionOutcome].
mod changes;
mod outcome;
mod quality;
pub use changes::changed_files;
pub use outcome::ExecutionOutcome;
pub use quality::with_quality;
