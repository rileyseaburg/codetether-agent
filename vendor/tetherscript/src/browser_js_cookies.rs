//! JavaScript `document.cookie` projection state.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::native;
use crate::js::{self, JsValue};

#[path = "browser_js_cookies/delete.rs"]
mod delete;
#[path = "browser_js_cookies/document.rs"]
mod document;
#[path = "browser_js_cookies/events.rs"]
mod events;
#[path = "browser_js_cookies/object.rs"]
mod object;
#[path = "browser_js_cookies/options.rs"]
mod options;
#[path = "browser_js_cookies/projection.rs"]
mod projection;
#[path = "browser_js_cookies/read.rs"]
mod read;
#[path = "browser_js_cookies/record.rs"]
mod record;
#[path = "browser_js_cookies/state.rs"]
mod state;
#[path = "browser_js_cookies/thenable.rs"]
mod thenable;
#[path = "browser_js_cookies/update.rs"]
mod update;
#[path = "browser_js_cookies/write.rs"]
mod write;

#[cfg(test)]
#[path = "browser_js_cookies/tests.rs"]
mod tests;
#[cfg(test)]
#[path = "browser_js_cookies/tests_events.rs"]
mod tests_events;

thread_local! {
    static COOKIE_JAR: RefCell<Vec<(String, String)>> = const { RefCell::new(Vec::new()) };
    static COOKIE_MUTATIONS: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

pub(crate) use document::set_document_cookie;
pub(crate) use object::create as store_object;
pub(crate) use state::{cookie_string, mutations, reset, seed, visible_pairs};
