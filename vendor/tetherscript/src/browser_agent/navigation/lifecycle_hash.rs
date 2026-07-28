//! Same-document hash lifecycle handling.

use crate::browser_agent::navigation::lifecycle_js::{self, JsNavigationEvent};
use crate::browser_agent::navigation::result::NavigationKind;
use crate::browser_agent::page::BrowserPage;

pub(crate) fn hashchange(
    page: &mut BrowserPage,
    action: &str,
    kind: NavigationKind,
    from: &str,
    to: &str,
) {
    super::lifecycle_log::event(page, "hashchange", action, kind, from, to);
    lifecycle_js::dispatch(
        page,
        JsNavigationEvent::HashChange {
            old_url: from,
            new_url: to,
        },
    );
}
