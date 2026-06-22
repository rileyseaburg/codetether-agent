//! Maps tree-sitter highlight capture names to RGB colors.
//!
//! Capture names come from `tree-sitter-rust`'s `highlights.scm` (e.g.
//! `keyword`, `function`, `string`, `type`). We map the leading segment to a
//! terminal-friendly color; unknown captures get no color (theme default).

/// Returns the RGB color for a highlight capture name, or `None` if unstyled.
///
/// Only the portion before the first `.` is significant (e.g. `function.method`
/// maps the same as `function`).
pub fn capture_color(name: &str) -> Option<(u8, u8, u8)> {
    let base = name.split('.').next().unwrap_or(name);
    let color = match base {
        "keyword" => (197, 134, 192),                  // purple
        "function" | "method" => (220, 220, 170),      // yellow
        "type" | "constructor" => (78, 201, 176),      // teal
        "string" => (206, 145, 120),                   // orange-brown
        "number" | "constant" => (181, 206, 168),      // light green
        "comment" => (106, 153, 85),                   // green
        "property" | "field" => (156, 220, 254),       // light blue
        "variable" => (156, 220, 254),                 // light blue
        "operator" | "punctuation" => (212, 212, 212), // light gray
        _ => return None,
    };
    Some(color)
}
