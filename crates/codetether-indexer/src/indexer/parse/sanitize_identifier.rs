//! sanitize identifier for codebase indexing.

pub(in crate::indexer) fn sanitize_identifier(input: &str) -> Option<String> {
    let mut out = String::new();
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' || ch == '$' {
            out.push(ch);
        } else {
            break;
        }
    }

    if out.is_empty() { None } else { Some(out) }
}
