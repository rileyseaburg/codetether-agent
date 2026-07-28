use super::*;
use crate::browser_agent::{BrowserContext, BrowserPage};

#[test]
fn page_permissions_are_origin_scoped() {
    let mut page = BrowserPage::from_html("https://app.test/path", "");
    assert_eq!(
        page.permission_state("https://app.test/other", BrowserPermission::Geolocation),
        PermissionState::Prompt
    );
    page.grant_permission("https://app.test/other", BrowserPermission::Geolocation);
    page.deny_permission("https://other.test", BrowserPermission::Geolocation);
    assert_eq!(
        page.permission_state("https://app.test", BrowserPermission::Geolocation),
        PermissionState::Granted
    );
}

#[test]
fn context_permissions_apply_to_pages_and_stay_isolated() {
    let mut first = BrowserContext::new();
    let second = BrowserContext::new();
    first.grant_permission("https://app.test", BrowserPermission::Notifications);
    let index = first.new_page(BrowserPage::from_html("https://app.test", ""));
    assert_eq!(
        first
            .page(index)
            .unwrap()
            .permission_state("https://app.test", BrowserPermission::Notifications),
        PermissionState::Granted
    );
    assert_eq!(
        second.permission_state("https://app.test", BrowserPermission::Notifications),
        PermissionState::Prompt
    );
}
