//! Native registry integration for the shared autonomous tool policy.
use super::*;
use crate::tool::ToolRegistry;

#[test]
fn read_only_registry_removes_mutating_tools() {
    let mut registry = ToolRegistry::with_defaults();
    restrict_registry(&mut registry, true);
    for denied in ["write", "edit", "multiedit", "patch", "bash", "ralph", "go"] {
        assert!(!registry.contains(denied), "{denied} should be blocked");
    }
    assert!(registry.contains("read"));
}
