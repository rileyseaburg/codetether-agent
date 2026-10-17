//! LSP linter extensions definitions and helpers.

/// Returns all file extensions handled by a named linter (built-in defaults).
pub fn linter_extensions(name: &str) -> &'static [&'static str] {
    match name {
        "eslint" => &["js", "jsx", "ts", "tsx", "mjs", "cjs"],
        "biome" => &["js", "jsx", "ts", "tsx", "json", "css"],
        "ruff" => &["py", "pyi"],
        "stylelint" => &["css", "scss", "less"],
        _ => super::tetherscript::linter_extensions(name),
    }
}
