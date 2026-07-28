//! Element properties used by selector healing.

/// Properties fingerprint of an element for similarity matching.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ElementProps {
    pub tag: String,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub role: Option<String>,
    pub text: Option<String>,
    pub href: Option<String>,
    pub label: Option<String>,
    pub parent_tag: Option<String>,
    pub position_hint: Option<usize>,
}
