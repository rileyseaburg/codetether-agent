//! Repository-documentation source-of-truth directives.

const PHRASES: &[&str] = &[
    "its in the docs",
    "it is in the docs",
    "use the repository docs",
    "use repository docs",
    "use the repository documentation",
    "use repository documentation",
    "use the docs as the source of truth",
    "docs are the source of truth",
    "repository docs are the source of truth",
    "repository documentation is the source of truth",
    "documentation is the source of truth",
    "look in the docs",
];

/// Detect an explicit instruction to use repository documentation alone.
pub(super) fn repository_only(text: &str) -> bool {
    let text = ["please ", "remember "]
        .iter()
        .find_map(|prefix| text.strip_prefix(prefix))
        .unwrap_or(text);
    PHRASES
        .iter()
        .any(|phrase| text == *phrase || text.starts_with(&format!("{phrase} ")))
}
