//! Reversible subtree lifecycle orchestration for durable child agents.

#[path = "thread_lifecycle/close.rs"]
mod close;
#[path = "thread_lifecycle/result.rs"]
mod result;
#[path = "thread_lifecycle/resume.rs"]
mod resume;
#[path = "thread_lifecycle/shutdown.rs"]
mod shutdown;
#[path = "thread_lifecycle/wait.rs"]
mod wait;

pub(super) use close::run as close;
pub(super) use resume::run as resume;

#[cfg(test)]
#[path = "thread_lifecycle/subtree_close_tests.rs"]
mod subtree_close_tests;
#[cfg(test)]
#[path = "thread_lifecycle/subtree_resume_tests.rs"]
mod subtree_resume_tests;
#[cfg(test)]
#[path = "thread_lifecycle/subtree_test_support.rs"]
mod subtree_test_support;
#[cfg(test)]
#[path = "thread_lifecycle/subtree_transition_tests.rs"]
mod subtree_transition_tests;
