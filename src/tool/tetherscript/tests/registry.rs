use crate::tool::ToolRegistry;

#[test]
#[cfg(feature = "tetherscript")]
fn default_registry_exposes_tetherscript_plugin() {
    assert!(ToolRegistry::with_defaults().contains("tetherscript_plugin"));
}
