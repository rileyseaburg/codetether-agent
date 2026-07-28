//! Internal navigation commits from default actions.

use crate::browser_agent::navigation::load_api;
use crate::browser_agent::navigation::result::{NavigationKind, NavigationResult};
use crate::browser_agent::page::BrowserPage;

pub(crate) fn commit_document(
    page: &mut BrowserPage,
    url: String,
    action: &str,
) -> NavigationResult {
    let from = page.session.url.clone();
    if let Some(hash) = super::url_kind::same_document_hash(&page.session.url, &url) {
        page.session.set_hash(hash);
        let result = load_api::push(page, action, NavigationKind::SameDocument);
        super::lifecycle::finish_commit(page, action, result.kind, &from, &result.navigation.url);
        return result;
    }
    super::lifecycle::leave(
        page,
        action,
        NavigationKind::DocumentReplacement,
        &from,
        &url,
    );
    page.session.goto_html(url, "");
    page.runtime = None;
    let result = load_api::push(page, action, NavigationKind::DocumentReplacement);
    super::lifecycle::finish_commit(page, action, result.kind, &from, &result.navigation.url);
    result
}
