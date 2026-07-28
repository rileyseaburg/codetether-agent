use super::*;
use crate::browser_agent::BrowserPage;

#[path = "tests_notification/close.rs"]
mod close;
#[path = "tests_notification/constructor.rs"]
mod constructor;
#[path = "tests_notification/request.rs"]
mod request;
#[path = "tests_notification/static_state.rs"]
mod static_state;

pub(super) fn page(state: PermissionState) -> BrowserPage {
    let mut page = BrowserPage::from_html("https://app.test", "<p id='out'></p>");
    match state {
        PermissionState::Granted => {
            page.grant_permission("https://app.test", BrowserPermission::Notifications);
        }
        PermissionState::Denied => {
            page.deny_permission("https://app.test", BrowserPermission::Notifications);
        }
        PermissionState::Prompt => {}
    }
    page
}

pub(super) fn value(mut page: BrowserPage, script: &str) -> String {
    page.eval_js(script).unwrap().display()
}
