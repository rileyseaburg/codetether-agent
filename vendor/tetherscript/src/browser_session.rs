//! Persistent in-process browser session state.
//!
//! `BrowserSession` is the deterministic page state model used by the
//! tetherscript browser implementation. It keeps the mutable page/session state
//! that agents need between navigation calls while reusing the lightweight
//! HTML/CSS parser in `browser`.

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::browser::{
    extract_embedded_css, parse_html, render_document_to_raster, Document, RasterImage,
    RenderOptions,
};
use crate::browser_cookie;
pub use crate::browser_cookie::{Cookie, SameSite};
use crate::browser_js::{eval_with_dom_state, run_html_scripts_with_state, BrowserJsResult};
#[path = "browser_session_console.rs"]
mod browser_session_console;
#[path = "browser_session_js_state.rs"]
mod browser_session_js_state;

const BLANK_URL: &str = "about:blank";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsoleEvent {
    pub level: String,
    pub message: String,
    pub timestamp_ms: u128,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkEvent {
    pub method: String,
    pub url: String,
    pub status: Option<u16>,
    pub route_result: Option<String>,
    pub timestamp_ms: u128,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceEvent {
    pub action: String,
    pub detail: String,
    pub timestamp_ms: u128,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrollState {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone, PartialEq)]
struct HistoryEntry {
    url: String,
    html: String,
    css: String,
    document: Document,
    focus: Option<String>,
    scroll: ScrollState,
}

/// Persistent browser session model for deterministic/offline navigation.
#[derive(Debug, Clone, PartialEq)]
pub struct BrowserSession {
    pub url: String,
    pub history: Vec<String>,
    pub document: Document,
    pub html: String,
    pub css: String,
    pub console: Vec<ConsoleEvent>,
    pub network: Vec<NetworkEvent>,
    pub storage: HashMap<String, String>,
    pub cookies: Vec<Cookie>,
    pub local_storage: HashMap<String, HashMap<String, String>>,
    pub session_storage: HashMap<String, HashMap<String, String>>,
    pub focus: Option<String>,
    pub scroll: ScrollState,
    pub trace: Vec<TraceEvent>,
    pub offline: bool,
    history_entries: Vec<HistoryEntry>,
    history_index: usize,
}

impl Default for BrowserSession {
    fn default() -> Self {
        Self::new()
    }
}

impl BrowserSession {
    /// Create a blank persistent browser session.
    pub fn new() -> Self {
        let document = parse_html("");
        let entry = HistoryEntry {
            url: BLANK_URL.to_string(),
            html: String::new(),
            css: String::new(),
            document: document.clone(),
            focus: None,
            scroll: ScrollState { x: 0, y: 0 },
        };
        Self {
            url: BLANK_URL.to_string(),
            history: vec![BLANK_URL.to_string()],
            document,
            html: String::new(),
            css: String::new(),
            console: Vec::new(),
            network: Vec::new(),
            storage: HashMap::new(),
            cookies: Vec::new(),
            local_storage: HashMap::new(),
            session_storage: HashMap::new(),
            focus: None,
            scroll: ScrollState { x: 0, y: 0 },
            trace: vec![TraceEvent::new("new", BLANK_URL)],
            offline: false,
            history_entries: vec![entry],
            history_index: 0,
        }
    }

    /// Load HTML into the current history entry without changing the current URL.
    pub fn load_html(&mut self, html: impl Into<String>) {
        let html = html.into();
        self.replace_current(self.url.clone(), html, String::new(), "load_html");
    }

    /// Navigate to an in-memory HTML page and append it to session history.
    pub fn goto_html(&mut self, url: impl Into<String>, html: impl Into<String>) {
        let url = url.into();
        let html = html.into();
        self.trace.push(TraceEvent::new("beforeunload", &self.url));
        self.trace.push(TraceEvent::new("unload", &self.url));
        self.truncate_forward_history();
        let entry = Self::entry_from_parts(url.clone(), html, String::new());
        self.history_entries.push(entry);
        self.history_index = self.history_entries.len() - 1;
        self.apply_history_entry();
        self.network.push(NetworkEvent::new("GET", &url, Some(200)));
        self.trace.push(TraceEvent::new("document_replace", &url));
        self.trace.push(TraceEvent::new("goto_html", &url));
    }

    /// Enable or disable offline mode. Offline mode is persistent session state;
    /// callers can inspect it before attempting external navigation.
    pub fn offline(&mut self, offline: bool) {
        self.offline = offline;
        self.trace.push(TraceEvent::new(
            "offline",
            if offline { "true" } else { "false" },
        ));
    }

    /// Reload the current in-memory page from its stored history entry.
    pub fn reload(&mut self) {
        self.trace.push(TraceEvent::new("beforeunload", &self.url));
        self.trace.push(TraceEvent::new("unload", &self.url));
        self.apply_history_entry();
        self.trace.push(TraceEvent::new("reload", &self.url));
    }

    /// Navigate to a new fragment on the current document without replacing it.
    pub fn set_hash(&mut self, hash: impl Into<String>) {
        let mut hash = hash.into();
        if !hash.is_empty() && !hash.starts_with('#') {
            hash = format!("#{}", hash);
        }
        let old_url = self.url.clone();
        let base = self.url.split('#').next().unwrap_or(&self.url).to_string();
        let next = format!("{}{}", base, hash);
        if next != old_url {
            self.url = next.clone();
            if let Some(entry) = self.history_entries.get_mut(self.history_index) {
                entry.url = next.clone();
            }
            self.history = self
                .history_entries
                .iter()
                .map(|entry| entry.url.clone())
                .collect();
            self.trace.push(TraceEvent::new(
                "hashchange",
                format!("{} -> {}", old_url, next),
            ));
        }
    }

    /// Store a cookie scoped from the current page URL.
    pub fn set_cookie(&mut self, cookie: impl AsRef<str>) -> Result<(), String> {
        browser_cookie::set_server_cookie(&mut self.cookies, cookie.as_ref(), &self.url)?;
        self.trace.push(TraceEvent::new("set_cookie", &self.url));
        Ok(())
    }

    /// Return cookies visible to `url` as a Cookie header string.
    pub fn cookie_header(&self, url: &str) -> String {
        browser_cookie::cookie_header(&self.cookies, url)
    }

    /// Return request cookies after modeled SameSite filtering.
    pub fn cookie_header_for_request(&self, url: &str, initiator_url: &str) -> String {
        browser_cookie::request_cookie_header(&self.cookies, url, initiator_url)
    }

    pub fn set_local_storage(&mut self, key: impl Into<String>, value: impl Into<String>) {
        let origin = browser_cookie::storage_origin(&self.url);
        self.local_storage
            .entry(origin.clone())
            .or_default()
            .insert(key.into(), value.into());
        self.trace.push(TraceEvent::new("local_storage", origin));
    }

    pub fn local_storage_item(&self, key: &str) -> Option<&str> {
        let origin = browser_cookie::storage_origin(&self.url);
        self.local_storage
            .get(&origin)
            .and_then(|items| items.get(key))
            .map(String::as_str)
    }

    pub fn set_session_storage(&mut self, key: impl Into<String>, value: impl Into<String>) {
        let origin = browser_cookie::storage_origin(&self.url);
        self.session_storage
            .entry(origin.clone())
            .or_default()
            .insert(key.into(), value.into());
        self.trace.push(TraceEvent::new("session_storage", origin));
    }

    pub fn session_storage_item(&self, key: &str) -> Option<&str> {
        let origin = browser_cookie::storage_origin(&self.url);
        self.session_storage
            .get(&origin)
            .and_then(|items| items.get(key))
            .map(String::as_str)
    }

    /// Execute all inline scripts in the current page and persist DOM/session side effects.
    pub fn run_scripts(&mut self) -> Result<(), String> {
        let state = self.browser_js_state();
        let result = run_html_scripts_with_state(&self.html, state)?;
        self.apply_browser_js_result(result, "run_scripts")
            .map(|_| ())
    }

    /// Evaluate JavaScript in the current page context and persist DOM/session side effects.
    pub fn eval_js(&mut self, script: &str) -> Result<crate::js::JsValue, String> {
        let state = self.browser_js_state();
        let result = eval_with_dom_state(&self.html, script, state)?;
        self.apply_browser_js_result(result, "eval_js")
    }

    /// Render the current document into a deterministic native RGBA framebuffer.
    pub fn render_raster(
        &self,
        viewport_width: i64,
        viewport_height: Option<i64>,
        scale: usize,
    ) -> Result<RasterImage, String> {
        render_document_to_raster(
            &self.document,
            &self.css,
            RenderOptions {
                viewport_width,
                viewport_height,
                scale,
                ..RenderOptions::default()
            },
        )
    }

    /// Render the current document to binary PPM bytes for simple file export.
    pub fn render_ppm(
        &self,
        viewport_width: i64,
        viewport_height: Option<i64>,
        scale: usize,
    ) -> Result<Vec<u8>, String> {
        self.render_raster(viewport_width, viewport_height, scale)
            .map(|image| image.to_ppm())
    }

    /// Move backward in history. Returns true if navigation occurred.
    pub fn back(&mut self) -> bool {
        if self.history_index == 0 {
            self.trace.push(TraceEvent::new("back", "at-start"));
            return false;
        }
        self.persist_current_view_state();
        self.history_index -= 1;
        self.apply_history_entry();
        self.trace.push(TraceEvent::new("back", &self.url));
        true
    }

    /// Move forward in history. Returns true if navigation occurred.
    pub fn forward(&mut self) -> bool {
        if self.history_index + 1 >= self.history_entries.len() {
            self.trace.push(TraceEvent::new("forward", "at-end"));
            return false;
        }
        self.persist_current_view_state();
        self.history_index += 1;
        self.apply_history_entry();
        self.trace.push(TraceEvent::new("forward", &self.url));
        true
    }

    fn replace_current(&mut self, url: String, html: String, css: String, action: &str) {
        self.persist_current_view_state();
        self.history_entries[self.history_index] = Self::entry_from_parts(url.clone(), html, css);
        self.apply_history_entry();
        self.trace.push(TraceEvent::new(action, &url));
    }

    fn entry_from_parts(url: String, html: String, css: String) -> HistoryEntry {
        let document = parse_html(&html);
        let embedded_css = extract_embedded_css(&document);
        let css = if css.trim().is_empty() {
            embedded_css
        } else if embedded_css.trim().is_empty() {
            css
        } else {
            format!("{}\n{}", embedded_css, css)
        };
        HistoryEntry {
            url,
            html,
            css,
            document,
            focus: None,
            scroll: ScrollState { x: 0, y: 0 },
        }
    }

    fn apply_history_entry(&mut self) {
        let entry = self.history_entries[self.history_index].clone();
        self.url = entry.url;
        self.html = entry.html;
        self.css = entry.css;
        self.document = entry.document;
        self.focus = entry.focus;
        self.scroll = entry.scroll;
        self.history = self
            .history_entries
            .iter()
            .map(|entry| entry.url.clone())
            .collect();
    }

    fn persist_current_view_state(&mut self) {
        if let Some(entry) = self.history_entries.get_mut(self.history_index) {
            entry.focus = self.focus.clone();
            entry.scroll = self.scroll.clone();
        }
    }

    fn truncate_forward_history(&mut self) {
        self.persist_current_view_state();
        self.history_entries.truncate(self.history_index + 1);
    }

    pub(crate) fn apply_browser_js_result(
        &mut self,
        result: BrowserJsResult,
        action: &str,
    ) -> Result<crate::js::JsValue, String> {
        let BrowserJsResult {
            document,
            html,
            value,
            console,
            state,
            network,
            ..
        } = result;
        let css = combine_embedded_and_external_css(&document, &self.external_css());
        self.persist_current_view_state();
        if let Some(entry) = self.history_entries.get_mut(self.history_index) {
            entry.html = html.clone();
            entry.css = css.clone();
            entry.document = document.clone();
        }
        self.html = html;
        self.css = css;
        self.document = document;
        self.apply_browser_js_state(state);
        self.console
            .extend(console.into_iter().map(browser_session_console::event));
        self.network.extend(network.into_iter().map(|event| {
            NetworkEvent::with_route_result(
                event.method,
                event.url,
                event.status,
                event.route_result,
            )
        }));
        self.trace.push(TraceEvent::new(action, &self.url));
        Ok(value)
    }

    fn external_css(&self) -> String {
        let embedded = extract_embedded_css(&self.document);
        if embedded.trim().is_empty() {
            return self.css.clone();
        }
        if self.css == embedded {
            return String::new();
        }
        let combined_prefix = format!("{}\n", embedded);
        if let Some(external) = self.css.strip_prefix(&combined_prefix) {
            return external.to_string();
        }
        self.css.clone()
    }
}

impl ConsoleEvent {
    pub fn new(level: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            level: level.into(),
            message: message.into(),
            timestamp_ms: now_ms(),
        }
    }
}

fn combine_embedded_and_external_css(document: &Document, external_css: &str) -> String {
    let embedded_css = extract_embedded_css(document);
    if external_css.trim().is_empty() {
        embedded_css
    } else if embedded_css.trim().is_empty() {
        external_css.to_string()
    } else {
        format!("{}\n{}", embedded_css, external_css)
    }
}

impl NetworkEvent {
    pub fn new(method: impl Into<String>, url: impl Into<String>, status: Option<u16>) -> Self {
        Self::with_route_result(method, url, status, None)
    }

    pub fn with_route_result(
        method: impl Into<String>,
        url: impl Into<String>,
        status: Option<u16>,
        route_result: Option<String>,
    ) -> Self {
        Self {
            method: method.into(),
            url: url.into(),
            status,
            route_result,
            timestamp_ms: now_ms(),
        }
    }
}

impl TraceEvent {
    pub fn new(action: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            action: action.into(),
            detail: detail.into(),
            timestamp_ms: now_ms(),
        }
    }
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::browser::query_selector;

    #[test]
    fn load_html_updates_current_document_and_css() {
        let mut session = BrowserSession::new();
        session.load_html("<style>p { color: red; }</style><p id='msg'>Hi</p>");

        assert_eq!(session.url, BLANK_URL);
        assert_eq!(session.history, vec![BLANK_URL.to_string()]);
        assert!(session.css.contains("color: red"));
        assert_eq!(query_selector(&session.document, "#msg").len(), 1);
    }

    #[test]
    fn goto_back_forward_preserve_view_state() {
        let mut session = BrowserSession::new();
        session.goto_html("mem://one", "<main id='one'></main>");
        session.focus = Some("#one".to_string());
        session.scroll = ScrollState { x: 4, y: 8 };
        session.goto_html("mem://two", "<main id='two'></main>");

        assert_eq!(session.history, vec![BLANK_URL, "mem://one", "mem://two"]);
        assert!(session.back());
        assert_eq!(session.url, "mem://one");
        assert_eq!(session.focus.as_deref(), Some("#one"));
        assert_eq!(session.scroll, ScrollState { x: 4, y: 8 });
        assert!(session.forward());
        assert_eq!(session.url, "mem://two");
    }

    #[test]
    fn new_navigation_truncates_forward_history() {
        let mut session = BrowserSession::new();
        session.goto_html("mem://one", "one");
        session.goto_html("mem://two", "two");
        assert!(session.back());
        session.goto_html("mem://three", "three");

        assert_eq!(session.history, vec![BLANK_URL, "mem://one", "mem://three"]);
        assert!(!session.forward());
    }

    #[test]
    fn hash_navigation_does_not_replace_document() {
        let mut session = BrowserSession::new();
        session.goto_html("https://example.test/app", "<main id='app'></main>");
        let original = session.document.clone();

        session.set_hash("details");

        assert_eq!(session.url, "https://example.test/app#details");
        assert_eq!(session.document, original);
        assert!(session
            .trace
            .iter()
            .any(|event| event.action == "hashchange"));
    }

    #[test]
    fn cookies_and_storage_are_origin_scoped() {
        let mut session = BrowserSession::new();
        session.goto_html("https://example.test/app", "");
        session
            .set_cookie("sid=abc; Path=/; Secure; HttpOnly")
            .unwrap();
        session.set_local_storage("token", "one");
        session.set_session_storage("tab", "a");

        assert_eq!(
            session.cookie_header("https://example.test/app/page"),
            "sid=abc"
        );
        assert_eq!(session.cookie_header("http://example.test/app/page"), "");
        assert_eq!(session.local_storage_item("token"), Some("one"));
        assert_eq!(session.session_storage_item("tab"), Some("a"));

        session.goto_html("https://other.test/app", "");
        assert_eq!(session.local_storage_item("token"), None);
        assert_eq!(session.session_storage_item("tab"), None);
    }

    #[test]
    fn eval_js_persists_local_storage_and_document_cookie() {
        let mut session = BrowserSession::new();
        session.goto_html("https://example.test/app", "<main id='app'></main>");

        session
            .eval_js("localStorage.setItem('token', 'one'); document.cookie = 'sid=abc'; 'stored';")
            .unwrap();
        let value = session
            .eval_js("localStorage.getItem('token') + ':' + document.cookie")
            .unwrap();

        assert_eq!(value, crate::js::JsValue::String("one:sid=abc".into()));
        assert_eq!(session.local_storage_item("token"), Some("one"));
        assert_eq!(
            session.cookie_header("https://example.test/app/page"),
            "sid=abc"
        );
    }

    #[test]
    fn eval_js_appends_fetch_and_xhr_network_events() {
        let mut session = BrowserSession::new();
        session.goto_html("https://example.test/app", "<main></main>");
        let initial_len = session.network.len();

        session
            .eval_js(
                "fetch('/api/data'); \
                 let xhr = XMLHttpRequest(); \
                 xhr.open('post', '/api/xhr'); \
                 xhr.send('body');",
            )
            .unwrap();

        let appended = &session.network[initial_len..];
        assert!(appended
            .iter()
            .any(|event| event.method == "GET" && event.url.ends_with("/api/data")));
        assert!(appended
            .iter()
            .any(|event| event.method == "POST" && event.url.ends_with("/api/xhr")));
    }

    #[test]
    fn session_can_render_current_document_to_pixels() {
        let mut session = BrowserSession::new();
        session.goto_html(
            "https://example.test/app",
            "<main style='background: #ff0000; width: 2px; height: 2px'></main>",
        );

        let image = session.render_raster(4, Some(4), 2).unwrap();

        assert_eq!((image.width, image.height), (8, 8));
        assert_eq!(
            image.pixel(1, 1),
            Some(crate::browser::Rgba {
                r: 255,
                g: 0,
                b: 0,
                a: 255
            })
        );
        assert!(session
            .render_ppm(4, Some(4), 2)
            .unwrap()
            .starts_with(b"P6\n"));
    }
}
