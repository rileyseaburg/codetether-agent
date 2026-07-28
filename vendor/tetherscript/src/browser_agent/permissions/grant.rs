//! Stored origin-scoped permission decision.

use super::{BrowserPermission, PermissionState};

/// One origin-scoped permission decision.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::permissions::{
///     BrowserPermission, PermissionGrant, PermissionState,
/// };
///
/// let grant = PermissionGrant::new(
///     "https://example.test",
///     BrowserPermission::Geolocation,
///     PermissionState::Granted,
/// );
/// assert_eq!(grant.origin, "https://example.test");
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PermissionGrant {
    /// Serialized origin key.
    pub origin: String,
    /// Permission name.
    pub permission: BrowserPermission,
    /// Deterministic decision state.
    pub state: PermissionState,
}

impl PermissionGrant {
    /// Create a permission grant record.
    pub fn new(
        origin: impl Into<String>,
        permission: BrowserPermission,
        state: PermissionState,
    ) -> Self {
        Self {
            origin: origin.into(),
            permission,
            state,
        }
    }
}
