//! Internal page permission store helpers.

use super::{BrowserPermission, PermissionState, PermissionStore};
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::security::Origin;

impl BrowserPage {
    pub(crate) fn replace_permissions(&mut self, permissions: PermissionStore) {
        self.permissions = permissions;
    }

    pub(super) fn set_permission(
        &mut self,
        origin_url: impl AsRef<str>,
        permission: BrowserPermission,
        state: PermissionState,
    ) -> Origin {
        let origin = Origin::parse(origin_url.as_ref());
        self.permissions.set(&origin, permission, state);
        origin
    }
}
