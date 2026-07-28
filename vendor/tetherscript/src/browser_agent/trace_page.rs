//! Browser page methods for action trace snapshots.

use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::trace::{ActionSnapshot, ActionTraceEntry, PageTrace};

impl BrowserPage {
    /// Capture the current page state for action tracing.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tetherscript::browser_agent::BrowserPage;
    ///
    /// let page = BrowserPage::from_html("mem://trace", "<main>Hi</main>");
    /// let snapshot = page.capture_snapshot();
    ///
    /// assert_eq!(snapshot.url, "mem://trace");
    /// ```
    pub fn capture_snapshot(&self) -> ActionSnapshot {
        ActionSnapshot {
            url: self.session.url.clone(),
            html: self.session.html.clone(),
            scroll: self.session.scroll.clone(),
            focused_selector: self.session.focus.clone(),
        }
    }

    /// Append one labeled before/after action trace entry.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tetherscript::browser_agent::BrowserPage;
    ///
    /// let mut page = BrowserPage::from_html("mem://trace", "<main>Before</main>");
    /// let before = page.capture_snapshot();
    /// page.session.load_html("<main>After</main>");
    /// let after = page.capture_snapshot();
    ///
    /// page.append_trace_entry("load_html", before, after);
    /// assert_eq!(page.trace_entries().len(), 1);
    /// ```
    pub fn append_trace_entry(
        &mut self,
        label: impl Into<String>,
        before: ActionSnapshot,
        after: ActionSnapshot,
    ) {
        self.action_trace
            .push(ActionTraceEntry::new(label, before, after));
    }

    /// Return action trace entries in insertion order.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tetherscript::browser_agent::BrowserPage;
    ///
    /// let page = BrowserPage::from_html("mem://trace", "<main></main>");
    ///
    /// assert!(page.trace_entries().is_empty());
    /// ```
    pub fn trace_entries(&self) -> &[ActionTraceEntry] {
        self.action_trace.entries()
    }

    /// Return the page action trace object.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tetherscript::browser_agent::BrowserPage;
    ///
    /// let page = BrowserPage::from_html("mem://trace", "<main></main>");
    ///
    /// assert!(page.page_trace().entries().is_empty());
    /// ```
    pub fn page_trace(&self) -> &PageTrace {
        &self.action_trace
    }
}
