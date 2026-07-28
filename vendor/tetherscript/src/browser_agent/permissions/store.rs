//! Deterministic permission grant store.

use super::PermissionGrant;

/// Origin-scoped browser permission decisions.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::permissions::PermissionStore;
///
/// assert!(PermissionStore::default().is_empty());
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PermissionStore {
    pub(super) grants: Vec<PermissionGrant>,
}
