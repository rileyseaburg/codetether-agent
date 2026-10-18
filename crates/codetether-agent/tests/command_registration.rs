//! Application registry coverage for the unified command tools.
use codetether_agent::tool::ToolRegistry;

#[test]
fn default_registry_exposes_unified_command_pair() {
    let registry = ToolRegistry::with_defaults();
    assert!(registry.contains("exec_command"));
    assert!(registry.contains("write_stdin"));
}
