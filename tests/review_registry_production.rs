//! Integration tests link the normal library, unlike tests compiled with cfg(test).
use codetether_agent::review::read_only_tools;
use codetether_agent::tool::readonly::is_read_only;

#[test]
fn reviewer_registry_is_available_in_production_and_read_only() {
    let registry = read_only_tools();
    assert!(registry.contains("read"));
    for id in registry.list() {
        assert!(is_read_only(id), "reviewer exposed non-read-only tool {id}");
    }
    for denied in ["write", "edit", "multiedit", "patch", "bash", "question"] {
        assert!(!registry.contains(denied), "reviewer exposed {denied}");
    }
}
