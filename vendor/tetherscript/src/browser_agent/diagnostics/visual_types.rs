//! Visual evidence types for production debug reports.

use std::collections::BTreeMap;

use crate::browser_agent::BoundingBox;

/// Native style and layout evidence for one element.
///
/// # Examples
///
/// ```rust
/// use std::collections::BTreeMap;
/// use tetherscript::browser_agent::{BoundingBox, VisualElementEvidence};
///
/// let evidence = VisualElementEvidence {
///     selector_candidates: vec!["#root".into()],
///     tag: "main".into(),
///     text: "Ready".into(),
///     bounds: BoundingBox { x: 0, y: 0, width: 10, height: 5 },
///     visible: true,
///     computed_styles: BTreeMap::from([("display".into(), "block".into())]),
/// };
/// assert!(evidence.visible);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VisualElementEvidence {
    /// Stable selectors an agent can use to re-locate the element.
    pub selector_candidates: Vec<String>,
    /// Lowercase element tag name.
    pub tag: String,
    /// Normalized visible text from the element subtree.
    pub text: String,
    /// Current layout bounds in CSS pixels.
    pub bounds: BoundingBox,
    /// Whether the element has a non-empty visible layout box.
    pub visible: bool,
    /// Computed CSS properties after cascade and page media filtering.
    pub computed_styles: BTreeMap<String, String>,
}
