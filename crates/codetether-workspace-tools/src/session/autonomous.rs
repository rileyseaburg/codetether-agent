//! Interactive-tool restrictions for autonomous sessions.

use crate::Registry;

const INTERACTIVE_TOOLS: &[&str] = &[
    "question",
    "confirm_edit",
    "confirm_multiedit",
    "voice_input",
];

/// Remove tools that require live user interaction from an autonomous registry.
/// # Arguments
/// * `registry` — Session catalog to filter in place.
/// # Returns
/// Removes the interactive IDs while retaining all other registered tools.
/// # Examples
/// ```no_run
/// # fn apply(registry: &mut impl codetether_workspace_tools::Registry) {
/// codetether_workspace_tools::session::autonomous::restrict(registry);
/// # }
/// ```
pub fn restrict(registry: &mut impl Registry) {
    for tool in INTERACTIVE_TOOLS {
        registry.unregister(tool);
    }
}
