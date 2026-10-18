use super::autonomous::restrict;
use crate::tool::ToolRegistry;

#[test]
fn autonomous_registry_excludes_interactive_tools() {
    let mut registry = ToolRegistry::with_defaults();
    restrict(&mut registry);

    for tool in [
        "question",
        "confirm_edit",
        "confirm_multiedit",
        "voice_input",
    ] {
        assert!(!registry.contains(tool), "{tool} must not be available");
    }
    assert!(registry.contains("edit"));
}
