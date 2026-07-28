//! Permission decision state.

/// Deterministic permission decision for one origin and permission.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::permissions::PermissionState;
///
/// assert_eq!(PermissionState::default(), PermissionState::Prompt);
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PermissionState {
    /// No explicit grant or denial has been configured.
    #[default]
    Prompt,
    /// Access is allowed.
    Granted,
    /// Access is blocked.
    Denied,
}

impl PermissionState {
    /// Return the browser-facing permission state string.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Prompt => "prompt",
            Self::Granted => "granted",
            Self::Denied => "denied",
        }
    }
}
