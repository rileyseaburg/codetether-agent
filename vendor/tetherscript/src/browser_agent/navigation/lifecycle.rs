//! Navigation lifecycle sequencing for page commits and traversal.

use crate::browser_agent::navigation::lifecycle_js::{self, JsNavigationEvent};
use crate::browser_agent::navigation::result::NavigationKind;
use crate::browser_agent::page::BrowserPage;

pub(crate) fn leave(
    page: &mut BrowserPage,
    action: &str,
    kind: NavigationKind,
    from: &str,
    to: &str,
) {
    super::lifecycle_log::event(page, "beforeunload", action, kind, from, to);
    lifecycle_js::dispatch(page, JsNavigationEvent::BeforeUnload);
    super::lifecycle_log::event(page, "unload", action, kind, from, to);
    lifecycle_js::dispatch(page, JsNavigationEvent::Unload);
}

pub(crate) fn finish_commit(
    page: &mut BrowserPage,
    action: &str,
    kind: NavigationKind,
    from: &str,
    to: &str,
) {
    match kind {
        NavigationKind::DocumentReplacement => {
            super::lifecycle_log::event(page, "document_replace", action, kind, from, to)
        }
        NavigationKind::Reload => {
            super::lifecycle_log::event(page, "reload", action, kind, from, to)
        }
        NavigationKind::SameDocument => {
            super::lifecycle_hash::hashchange(page, action, kind, from, to)
        }
        NavigationKind::Initial => {}
    }
}

pub(crate) fn finish_history(
    page: &mut BrowserPage,
    action: &str,
    kind: NavigationKind,
    from: &str,
    to: &str,
) {
    super::lifecycle_log::event(page, "popstate", action, kind, from, to);
    lifecycle_js::dispatch(page, JsNavigationEvent::PopState);
    finish_commit(page, action, kind, from, to);
}

pub(crate) fn no_entry(page: &mut BrowserPage, action: &str, url: &str) {
    super::lifecycle_log::no_entry(page, action, url);
}
