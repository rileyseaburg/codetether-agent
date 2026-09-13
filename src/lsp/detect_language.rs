//! Map a file path to the language id used for server resolution.

/// Detect language from file extension.
///
/// # Examples
///
/// ```
/// use codetether_agent::lsp::detect_language_from_path;
///
/// assert_eq!(detect_language_from_path("src/main.rs"), Some("rust"));
/// assert_eq!(detect_language_from_path("plugin.tether"), Some("tetherscript"));
/// assert_eq!(detect_language_from_path("README"), None);
/// ```
pub fn detect_language_from_path(path: &str) -> Option<&'static str> {
    let ext = path.rsplit('.').next()?;
    match ext {
        "rs" => Some("rust"),
        "ts" | "tsx" => Some("typescript"),
        "js" | "jsx" => Some("javascript"),
        "py" => Some("python"),
        "go" => Some("go"),
        "c" => Some("c"),
        "cpp" | "cc" | "cxx" => Some("cpp"),
        "h" => Some("c"),
        "hpp" => Some("cpp"),
        "tether" | "kl" => Some(crate::lsp::tetherscript::TETHERSCRIPT_LANGUAGE),
        _ => None,
    }
}
