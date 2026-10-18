//! Existing native cognition regressions with isolated state and no installed tools.
use super::*;
use std::sync::Arc;
pub(crate) type CognitionRuntime = super::CognitionRuntime<TestRegistry, true>;
#[derive(Debug)]
pub(crate) struct TestRegistry;
impl TestRegistry {
    pub(crate) fn new() -> Self {
        Self
    }
}
impl codetether_cognition_core::execution::dispatch::Registry for TestRegistry {
    fn get(&self, _id: &str) -> Option<Arc<dyn codetether_tool_core::Tool>> {
        panic!("cognition fixtures do not install or consult a tool registry")
    }
}
mod budget_tests;
mod governance_quorum_tests;
mod governance_tests_support;
mod governance_veto_tests;
mod idle_reap_tests;
mod lifecycle_tests;
mod tests_request;
mod tests_support;
