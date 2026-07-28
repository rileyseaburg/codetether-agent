//! Shared document load commit helpers.

use crate::browser_agent::navigation::result::{NavigationKind, NavigationResult};
use crate::browser_agent::navigation_state::PageLoadState;
use crate::browser_agent::page::BrowserPage;

pub(crate) fn replace(
    page: &mut BrowserPage,
    action: &str,
    kind: NavigationKind,
) -> NavigationResult {
    page.navigation = page
        .navigation
        .next(page.session.url.clone(), action, PageLoadState::Load);
    page.history
        .replace_current(page.session.clone(), page.navigation.id, kind);
    NavigationResult::committed(page.navigation.clone(), kind)
}

pub(crate) fn push(page: &mut BrowserPage, action: &str, kind: NavigationKind) -> NavigationResult {
    page.navigation = page
        .navigation
        .next(page.session.url.clone(), action, PageLoadState::Load);
    page.history
        .push(page.session.clone(), page.navigation.id, kind);
    NavigationResult::committed(page.navigation.clone(), kind)
}
