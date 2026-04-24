use crate::tool::ToolRegistry;

#[test]
fn default_registry_exposes_kiln_plugin() {
    assert!(ToolRegistry::with_defaults().contains("kiln_plugin"));
}
