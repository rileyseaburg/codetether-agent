//! RustyRoad registration in the complete application tool registry.
use codetether_agent::tool::ToolRegistry;

#[test]
fn rustyroad_is_registered_in_standard_registries() {
    let registry = ToolRegistry::with_defaults();
    assert!(registry.get("rustyroad").is_some());
}
