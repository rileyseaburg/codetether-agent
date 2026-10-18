//! ends with source ext policy.

pub(super) fn ends_with_source_ext(s: &str) -> bool {
    [
        ".rs", ".ts", ".tsx", ".js", ".jsx", ".py", ".go", ".md", ".json", ".toml", ".yaml",
        ".yml", ".html", ".css", ".c", ".cpp", ".h", ".hpp",
    ]
    .iter()
    .any(|ext| s.ends_with(ext))
}
