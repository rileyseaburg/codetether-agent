//! Internal context permission propagation helpers.

use super::{BrowserPermission, PermissionState};
use crate::browser_agent::context::BrowserContext;
use crate::browser_agent::security::Origin;

impl BrowserContext {
    pub(super) fn set_permission(
        &mut self,
        origin_url: impl AsRef<str>,
        permission: BrowserPermission,
        state: PermissionState,
    ) -> Origin {
        let origin = Origin::parse(origin_url.as_ref());
        self.permissions.set(&origin, permission, state);
        self.apply_permissions();
        origin
    }

    pub(super) fn apply_permissions(&mut self) {
        for page in &mut self.pages {
            page.replace_permissions(self.permissions.clone());
        }
    }

    pub(super) fn apply_geolocation(&mut self) {
        for page in &mut self.pages {
            page.geolocation = self.geolocation.clone();
        }
    }
}
