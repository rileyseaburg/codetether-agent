//! Sandbox flag metadata for agent security policy.

/// Small deterministic subset of browser sandbox flags.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::SandboxFlags;
///
/// let flags = SandboxFlags::default();
/// assert!(flags.allow_scripts);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SandboxFlags {
    /// Whether scripts are allowed by metadata policy.
    pub allow_scripts: bool,
    /// Whether forms are allowed by metadata policy.
    pub allow_forms: bool,
    /// Whether same-origin treatment is allowed.
    pub allow_same_origin: bool,
}

impl Default for SandboxFlags {
    fn default() -> Self {
        Self {
            allow_scripts: true,
            allow_forms: true,
            allow_same_origin: true,
        }
    }
}
