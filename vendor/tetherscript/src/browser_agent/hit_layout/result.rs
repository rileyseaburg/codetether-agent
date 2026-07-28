//! Hit test result.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HitResult {
    pub x: i64,
    pub y: i64,
    pub tag: Option<String>,
    pub text: Option<String>,
    pub path: Vec<usize>,
}
