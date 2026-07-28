//! Public computed-style snapshot model.

use std::collections::BTreeMap;

/// Stable computed-style properties for one DOM element.
///
/// # Examples
///
/// ```rust
/// use std::collections::BTreeMap;
/// use tetherscript::browser_agent::page::cssom::ComputedStyle;
///
/// let mut properties = BTreeMap::new();
/// properties.insert("color".into(), "red".into());
/// let style = ComputedStyle { path: vec![0], tag: "main".into(), properties };
/// assert_eq!(style.get("COLOR"), Some("red"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComputedStyle {
    /// DOM child-index path from the document root.
    pub path: Vec<usize>,
    /// Lowercase element tag name.
    pub tag: String,
    /// Deterministically ordered CSS property values.
    pub properties: BTreeMap<String, String>,
}

impl ComputedStyle {
    /// Return a property value by CSS property name.
    ///
    /// # Arguments
    ///
    /// * `name` - CSS property name, matched case-insensitively.
    ///
    /// # Returns
    ///
    /// The property value when it exists on this snapshot.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::collections::BTreeMap;
    /// # use tetherscript::browser_agent::page::cssom::ComputedStyle;
    /// let style = ComputedStyle {
    ///     path: vec![0],
    ///     tag: "main".into(),
    ///     properties: BTreeMap::from([("display".into(), "block".into())]),
    /// };
    /// assert_eq!(style.get("display"), Some("block"));
    /// ```
    pub fn get(&self, name: &str) -> Option<&str> {
        self.properties
            .get(&name.trim().to_ascii_lowercase())
            .map(String::as_str)
    }

    pub(crate) fn property(&self, name: &str) -> Option<String> {
        self.get(name).map(str::to_string)
    }
}
