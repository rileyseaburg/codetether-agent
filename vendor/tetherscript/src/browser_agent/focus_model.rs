//! Public focus target metadata.

/// One keyboard-focusable element in page tab order.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::{BrowserPage, FocusTarget};
///
/// let page = BrowserPage::from_html("mem://page", "<button id='save'>Save</button>");
/// let target: FocusTarget = page.focus_order().remove(0);
/// assert_eq!(target.selector, "#save");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FocusTarget {
    /// Stable DOM child-index path from the document root.
    pub path: Vec<usize>,
    /// Human-readable focus key, preferring `#id` and falling back to `path:*`.
    pub selector: String,
    /// Lowercase element tag name.
    pub tag: String,
}
