//! Existing case-insensitive stub vocabulary.

pub(super) fn stub_marker_in_text(text: &str) -> Option<&'static str> {
    let lower = text.to_ascii_lowercase();
    let markers = [
        "todo",
        "fixme",
        "placeholder implementation",
        "<placeholder>",
        "[placeholder]",
        "{placeholder}",
        "not implemented",
        "fallback",
        "stub",
        "coming soon",
        "unimplemented!(",
        "todo!(",
        "throw new error(\"not implemented",
    ];
    markers.into_iter().find(|m| lower.contains(m))
}
