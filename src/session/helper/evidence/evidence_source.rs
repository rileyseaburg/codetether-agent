//! Concrete evidence-source detection for verification claims.

const SOURCES: &[&str] = &[
    "cargo ",
    "./",
    "artifact",
    "commit ",
    "http",
    "pod ",
    "job ",
    "namespace ",
    "diff",
    "source inspection",
    "not-run:",
];

pub(super) fn is_named(line: &str) -> bool {
    let lower = line.to_ascii_lowercase();
    SOURCES.iter().any(|source| lower.contains(source))
}
