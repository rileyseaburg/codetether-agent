//! Page storage view helpers for snapshots.

use std::collections::HashMap;

use crate::browser_agent::BrowserPage;
use crate::browser_cookie;
use crate::browser_session::Cookie;

pub(super) type StorageMap = HashMap<String, HashMap<String, String>>;

pub(super) fn shared_storage_view(page: &BrowserPage) -> (Vec<Cookie>, StorageMap) {
    page.context_state
        .as_ref()
        .map(|state| {
            let state = state.borrow();
            (
                browser_cookie::persistent_cookies(&state.cookies),
                state.local_storage.clone(),
            )
        })
        .unwrap_or_else(|| {
            (
                browser_cookie::persistent_cookies(&page.session.cookies),
                page.session.local_storage.clone(),
            )
        })
}
