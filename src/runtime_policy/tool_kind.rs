//! Tool side-effect classification.

/// Side-effect class used by [`crate::runtime_policy::RuntimeToolPolicy`].
///
/// # Examples
///
/// ```rust
/// use codetether_agent::runtime_policy::ToolKind;
///
/// assert_eq!(ToolKind::for_name("read"), ToolKind::ReadOnly);
/// assert_eq!(ToolKind::for_name("bash"), ToolKind::Mutating);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolKind {
    /// Known not to mutate local or remote state.
    ReadOnly,
    /// Known to be capable of mutating local or remote state.
    Mutating,
    /// Not classified; callers should treat it conservatively.
    Unknown,
}

impl ToolKind {
    /// Classify a tool name by its known side-effect behavior.
    pub fn for_name(tool_name: &str) -> Self {
        if matches!(tool_name, "bash" | "apply_patch" | "patch") {
            Self::Mutating
        } else if crate::tool::readonly::is_read_only(tool_name) {
            Self::ReadOnly
        } else {
            Self::Unknown
        }
    }
}
