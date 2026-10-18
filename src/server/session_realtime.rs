//! Native host binding for shared session realtime HTTP and execution.
#[path = "session_realtime/prompt_run.rs"]
mod prompt_run;
pub(super) use prompt_run::AppHost;
