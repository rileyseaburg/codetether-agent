//! Native integration tests for shared human continuation policy.
use codetether_goal_answer::continuation::requested;
#[path = "continuation_release.rs"]
mod release;
#[path = "continuation_resume_tests.rs"]
mod resume_tests;
#[path = "continuation_tests.rs"]
mod tests;
