//! Accessibility snapshot data structures.

/// A deterministic accessibility tree plus page-level focus order.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::{AccessibilitySnapshot, BrowserPage};
///
/// let page = BrowserPage::from_html("mem://a11y", "<button>Save</button>");
/// let snapshot: AccessibilitySnapshot = page.accessibility_snapshot();
/// assert_eq!(snapshot.roots[0].role, "button");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessibilitySnapshot {
    /// Root accessibility nodes in DOM order after hidden subtree filtering.
    pub roots: Vec<AccessibilityNode>,
    /// Focusable element selectors in deterministic keyboard order.
    pub focus_order: Vec<String>,
}

/// One node in a deterministic accessibility snapshot.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::BrowserPage;
///
/// let page = BrowserPage::from_html("mem://a11y", "<img alt='Logo'>");
/// let node = page.accessibility_snapshot().roots.remove(0);
/// assert_eq!(node.name, "Logo");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessibilityNode {
    /// Computed explicit or implicit ARIA role.
    pub role: String,
    /// Computed accessible name, or an empty string when unnamed.
    pub name: String,
    /// Lowercase DOM tag, or `#text` for text leaf nodes.
    pub tag: String,
    /// Stable DOM child-index path from the document root.
    pub path: Vec<usize>,
    /// Human-readable selector key, preferring `#id` for elements.
    pub selector: String,
    /// Whether this node participates in keyboard focus order.
    pub focusable: bool,
    /// Zero-based index in [`AccessibilitySnapshot::focus_order`].
    pub focus_index: Option<usize>,
    /// Common ARIA/native state values.
    pub states: AccessibilityState,
    /// Child accessibility nodes.
    pub children: Vec<AccessibilityNode>,
}

/// Common accessibility states surfaced on snapshot nodes.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AccessibilityState {
    /// `aria-checked`, native checkbox/radio state, or `mixed`.
    pub checked: Option<String>,
    /// Native `disabled` or `aria-disabled="true"`.
    pub disabled: bool,
    /// `aria-expanded` when present.
    pub expanded: Option<String>,
    /// `aria-selected` or native `selected` option state.
    pub selected: Option<String>,
    /// `aria-pressed`, including `mixed`.
    pub pressed: Option<String>,
}
