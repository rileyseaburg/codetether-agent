//! PRD response recovery: sanitize json.

use crate::{commas::remove_trailing_commas, comments::remove_line_comments};

/// Sanitize common LLM JSON mistakes.
pub(crate) fn sanitize_json(text: &str) -> String {
    let mut s = text.to_string();

    // Replace unicode curly quotes with straight quotes
    s = s
        .replace(['\u{201c}', '\u{201d}'], "\"") // right double
        .replace(['\u{2018}', '\u{2019}'], "'"); // right single

    // Remove single-line // comments (outside strings)
    s = remove_line_comments(&s);

    // Remove trailing commas before } or ]
    s = remove_trailing_commas(&s);

    s
}
