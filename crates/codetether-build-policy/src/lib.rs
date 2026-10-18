//! Build-mode intent detection and tool-first retry decisions.
//!
//! [build_request_requires_tool] uses the current request, earlier proposals
//! and existing workspace paths. [should_force_build_tool_first_retry] applies
//! the established retry limits. Helpers preserve the native loop's API.
mod affirmative;
mod execution;
mod history;
mod offered;
mod paths;
mod proposed;
mod request;
mod request_scope;
mod retry;
pub use affirmative::is_affirmative_build_followup;
pub use execution::looks_like_build_execution_request;
pub use offered::assistant_offered_next_step;
pub use paths::extract_candidate_file_paths;
pub use proposed::looks_like_proposed_change;
pub use request::build_request_requires_tool;
pub use retry::{is_build_agent, should_force_build_tool_first_retry};
#[cfg(test)]
mod tests;
