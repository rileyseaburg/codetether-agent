//! One position in the display-oriented agent hierarchy.

/// Maximum nesting depth. Depth 0 = child of the main chat, so this allows
/// up to five generations of agents-spawning-agents before spawns are denied.
pub const MAX_AGENT_DEPTH: u8 = 5;

/// One entry in the depth-first agent ordering.
///
/// # Examples
/// ```
/// let node = codetether_agent_tree::AgentNode { name: "reviewer".into(), depth: 0 };
/// assert_eq!(node.depth, 0);
/// ```
pub struct AgentNode {
    /// Agent map key (unique name).
    pub name: String,
    /// Nesting depth (0 = child of main chat).
    pub depth: u8,
}
