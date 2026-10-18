//! Parent-name access for in-memory agent display trees.
/// Minimal hierarchy fields for agents stored in a map keyed by name.
///
/// # Implementors
/// Expose the existing parent link and nesting depth without copying session state.
///
/// # Examples
/// ```
/// struct Child { parent: Option<String>, depth: u8 }
/// impl codetether_agent_tree::HierarchyEntry for Child {
///     fn parent_name(&self) -> Option<&str> { self.parent.as_deref() }
///     fn depth(&self) -> u8 { self.depth }
/// }
/// ```
pub trait HierarchyEntry {
    /// Parent map key, or None for a direct child of the main agent.
    fn parent_name(&self) -> Option<&str>;
    /// Zero-based depth below the main agent.
    fn depth(&self) -> u8;
}
