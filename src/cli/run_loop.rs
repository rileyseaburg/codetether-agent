//! Native session binding for crate-owned resume loops.
#[path = "run_loop_host.rs"]
mod host;
pub use codetether_run_resume::{execute_prompt_with_resume, execute_prompt_with_resume_events};
