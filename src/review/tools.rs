//! The reviewer's tool surface: the default registry minus every mutator.

use crate::tool::ToolRegistry;

/// The default registry with every mutating tool removed.
///
/// Delegates to the swarm read-only policy so the reviewer and read-only
/// swarm sub-agents can never drift apart on what "read-only" means.
pub fn read_only_tools() -> ToolRegistry {
    let mut registry = ToolRegistry::with_defaults();
    crate::swarm::tool_policy::restrict_registry(&mut registry, true);
    registry
}
