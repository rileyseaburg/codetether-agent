//! Interactive-tool restrictions for autonomous sessions.

use crate::tool::ToolRegistry;

const INTERACTIVE_TOOLS: &[&str] = &[
    "question",
    "confirm_edit",
    "confirm_multiedit",
    "voice_input",
];

/// Remove tools that require live user interaction from an autonomous registry.
pub(super) fn restrict(registry: &mut ToolRegistry) {
    for tool in INTERACTIVE_TOOLS {
        registry.unregister(tool);
    }
}
