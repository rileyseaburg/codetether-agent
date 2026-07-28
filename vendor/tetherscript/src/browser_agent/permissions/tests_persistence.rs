use super::*;
use crate::browser_agent::{BrowserContext, BrowserPage};

#[test]
fn context_snapshot_restores_permissions_and_geolocation() {
    let position = GeolocationPosition::new(1.0, 2.0, 3.0).unwrap();
    let mut context = BrowserContext::new();
    context.grant_permission("https://app.test", BrowserPermission::Camera);
    context.set_geolocation(position);
    let snapshot = context.snapshot_state();
    let mut restored = BrowserContext::new();
    restored.restore_state(snapshot).unwrap();
    assert_eq!(
        restored.permission_state("https://app.test", BrowserPermission::Camera),
        PermissionState::Granted
    );
    assert_eq!(
        restored.geolocation(),
        &GeolocationEmulation::Position(position)
    );
}

#[test]
fn page_snapshot_restores_permissions_and_geolocation_error() {
    let error = GeolocationError::new(GeolocationErrorCode::Timeout, "slow position");
    let mut page = BrowserPage::from_html("https://app.test", "");
    page.deny_permission("https://app.test", BrowserPermission::Microphone);
    page.set_geolocation_error(error.clone());
    let snapshot = page.snapshot_state();
    let mut restored = BrowserPage::new(Default::default());
    restored.restore_state(snapshot).unwrap();
    assert_eq!(
        restored.permission_state("https://app.test", BrowserPermission::Microphone),
        PermissionState::Denied
    );
    assert_eq!(restored.geolocation(), &GeolocationEmulation::Error(error));
}
