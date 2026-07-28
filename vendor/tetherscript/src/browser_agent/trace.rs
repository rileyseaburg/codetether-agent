//! Action trace state captured from an agent-controlled page.

#[path = "trace_page.rs"]
mod trace_page;

use crate::browser_session::ScrollState;

/// Immutable page state captured before or after an action.
///
/// # Examples
///
/// ```rust
/// use tetherscript::browser_agent::{ActionSnapshot, BrowserPage};
///
/// let page = BrowserPage::from_html("mem://trace", "<main>Hi</main>");
/// let snapshot: ActionSnapshot = page.capture_snapshot();
///
/// assert_eq!(snapshot.url, "mem://trace");
/// assert!(snapshot.html.contains("Hi"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionSnapshot {
    /// Current page URL.
    pub url: String,
    /// Current serialized page HTML.
    pub html: String,
    /// Current page scroll offset.
    pub scroll: ScrollState,
    /// Selector for the focused element, when the session knows it.
    pub focused_selector: Option<String>,
}

/// A labeled before/after action trace entry.
///
/// # Examples
///
/// ```rust
/// use tetherscript::browser_agent::BrowserPage;
///
/// let mut page = BrowserPage::from_html("mem://trace", "<p>Before</p>");
/// let before = page.capture_snapshot();
/// page.session.load_html("<p>After</p>");
/// let after = page.capture_snapshot();
///
/// page.append_trace_entry("load_html", before, after);
/// assert_eq!(page.trace_entries()[0].label, "load_html");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionTraceEntry {
    /// Stable action label supplied by the caller.
    pub label: String,
    /// Page state before the action.
    pub before: ActionSnapshot,
    /// Page state after the action.
    pub after: ActionSnapshot,
}

impl ActionTraceEntry {
    pub(crate) fn new(
        label: impl Into<String>,
        before: ActionSnapshot,
        after: ActionSnapshot,
    ) -> Self {
        Self {
            label: label.into(),
            before,
            after,
        }
    }
}

/// Ordered action trace for a single page.
///
/// # Examples
///
/// ```rust
/// use tetherscript::browser_agent::{BrowserPage, PageTrace};
///
/// let page = BrowserPage::from_html("mem://trace", "<main></main>");
/// let trace: &PageTrace = page.page_trace();
///
/// assert!(trace.entries().is_empty());
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PageTrace {
    entries: Vec<ActionTraceEntry>,
}

impl PageTrace {
    /// Return entries in deterministic insertion order.
    pub fn entries(&self) -> &[ActionTraceEntry] {
        &self.entries
    }

    pub(crate) fn push(&mut self, entry: ActionTraceEntry) {
        self.entries.push(entry);
    }
}
