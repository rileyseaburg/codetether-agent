//! Page event records exposed by the agent browser.

#[path = "events_capture.rs"]
mod events_capture;
#[path = "events_page.rs"]
mod events_page;
#[path = "events_push.rs"]
mod events_push;

/// Kind of event summarized in a deterministic page event log.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::PageEventKind;
///
/// let kind = PageEventKind::Console;
/// assert!(matches!(kind, PageEventKind::Console));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PageEventKind {
    /// A console entry captured from `console.log`.
    Console,
    /// A navigation lifecycle event observed by page navigation APIs.
    Navigation,
    /// A JavaScript execution error raised by page work.
    PageError,
    /// A network request observed by the deterministic host.
    Network,
}

/// JavaScript execution error captured at the page API boundary.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::PageErrorEvent;
///
/// let event = PageErrorEvent {
///     action: "page.eval_js".into(),
///     message: "missing function".into(),
/// };
/// assert_eq!(event.action, "page.eval_js");
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PageErrorEvent {
    /// Page action that produced the error.
    pub action: String,
    /// Error message returned by the JavaScript runtime or host bridge.
    pub message: String,
}

/// Stable event summary for agents and tests.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::{PageEventKind, PageEventSummary};
///
/// let event = PageEventSummary {
///     sequence: 0,
///     kind: PageEventKind::Console,
///     action: "log".into(),
///     message: "ready".into(),
/// };
/// assert_eq!(event.sequence, 0);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PageEventSummary {
    /// Sequence number assigned by the page, starting at zero.
    pub sequence: u64,
    /// Event category.
    pub kind: PageEventKind,
    /// Source action or event label.
    pub action: String,
    /// Human-readable event detail.
    pub message: String,
}

#[derive(Clone, Copy)]
pub(crate) struct PageEventCheckpoint {
    pub(crate) console_len: usize,
    pub(crate) network_len: usize,
}

impl PageEventCheckpoint {
    pub(crate) fn new(console_len: usize, network_len: usize) -> Self {
        Self {
            console_len,
            network_len,
        }
    }
}
