//! Resumable CLI prompt loops and exhaustion checkpoints.
//!
//! [execute_prompt_with_resume] runs the initial prompt and up to the configured
//! number of resume attempts. The event variant loads providers once first.
//! [RunSession] supplies native prompt execution while checkpoint storage and
//! thread-local budget state retain their existing shared owners.
mod checkpoint;
mod events;
mod host;
mod plain;
mod plan;
pub use checkpoint::{persist_exhaustion_checkpoint, resume_plan, should_checkpoint};
pub use events::execute_prompt_with_resume_events;
pub use host::RunSession;
pub use plain::execute_prompt_with_resume;
pub use plan::{RunResumePlan, validate_auto_continue};

mod selection;
mod selection_last;
mod source;
pub use selection::select_session;
pub use source::SessionSource;
