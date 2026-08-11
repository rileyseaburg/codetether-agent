//! Neutral byte-range captures produced by tree-sitter queries.

/// A named syntax capture and its half-open UTF-8 byte range.
///
/// Consumers can map the capture name to presentation-specific styling without
/// depending directly on tree-sitter types.
///
/// # Examples
///
/// ```rust
/// use codetether_rlm::AstCapture;
/// let capture = AstCapture {
///     name: "keyword".to_string(),
///     start_byte: 0,
///     end_byte: 2,
/// };
/// assert_eq!(capture.start_byte..capture.end_byte, 0..2);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstCapture {
    /// Tree-sitter query capture name, such as `keyword` or `function`.
    pub name: String,
    /// Inclusive start byte in the parsed source.
    pub start_byte: usize,
    /// Exclusive end byte in the parsed source.
    pub end_byte: usize,
}
