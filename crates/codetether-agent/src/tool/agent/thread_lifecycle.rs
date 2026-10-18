//! Application bindings for shared durable-child lifecycle orchestration.

codetether_agent_lifecycle::declare_application_lifecycle!(
    pub(super),
    crate::session::Session,
    crate::tool::agent::persistence,
    crate::tool::agent::residency,
    crate::tool::agent::store,
    crate::tool::agent::event_loop::live_trace::clear,
);

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
