use crate::tool::ToolRegistry;

#[test]
fn default_registry_exposes_tetherscript_plugin() {
    assert!(ToolRegistry::with_defaults().contains("tetherscript_plugin"));
}
