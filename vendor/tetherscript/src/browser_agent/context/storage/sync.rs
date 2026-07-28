//! Helpers for pushing shared storage into open pages.

use super::super::BrowserContext;

pub(super) fn sync_pages(context: &mut BrowserContext) {
    for page in &mut context.pages {
        page.sync_context_state_into_session();
        page.runtime = None;
    }
}
