//! Browser JavaScript state conversion for persistent sessions.

use std::collections::HashMap;

use crate::browser_js::BrowserJsState;
use crate::{browser_cookie, browser_session::BrowserSession};

impl BrowserSession {
    pub(crate) fn browser_js_state(&self) -> BrowserJsState {
        let origin = browser_cookie::storage_origin(&self.url);
        BrowserJsState {
            url: self.url.clone(),
            cookies: browser_cookie::document_cookie_pairs(&self.cookies, &self.url),
            cookie_jar: self.cookies.clone(),
            set_cookies: Vec::new(),
            local_storage: storage_pairs(self.local_storage.get(&origin)),
            session_storage: storage_pairs(self.session_storage.get(&origin)),
        }
    }

    pub(crate) fn apply_browser_js_state(&mut self, state: BrowserJsState) {
        if !state.url.is_empty() {
            self.url = state.url;
        }
        self.cookies = state.cookie_jar;
        browser_cookie::apply_document_cookies(&mut self.cookies, state.set_cookies, &self.url);
        let origin = browser_cookie::storage_origin(&self.url);
        self.local_storage
            .insert(origin.clone(), pairs_to_storage_map(state.local_storage));
        self.session_storage
            .insert(origin, pairs_to_storage_map(state.session_storage));
    }
}

fn storage_pairs(items: Option<&HashMap<String, String>>) -> Vec<(String, String)> {
    items
        .map(|items| {
            items
                .iter()
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect()
        })
        .unwrap_or_default()
}

fn pairs_to_storage_map(pairs: Vec<(String, String)>) -> HashMap<String, String> {
    pairs.into_iter().collect()
}
