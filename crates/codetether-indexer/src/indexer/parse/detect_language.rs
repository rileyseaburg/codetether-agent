//! detect language for codebase indexing.

use super::*;

pub(in crate::indexer) fn detect_language(path: &Path) -> String {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    match ext.as_str() {
        "rs" => "rust",
        "ts" | "tsx" => "typescript",
        "js" | "jsx" => "javascript",
        "py" => "python",
        "go" => "go",
        "java" => "java",
        "kt" => "kotlin",
        "c" | "h" => "c",
        "cpp" | "hpp" | "cc" | "cxx" => "cpp",
        "json" => "json",
        "yaml" | "yml" => "yaml",
        "toml" => "toml",
        "md" => "markdown",
        "sh" | "bash" | "zsh" => "shell",
        "proto" => "proto",
        "sql" => "sql",
        "html" => "html",
        "css" | "scss" => "css",
        _ => {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                match name {
                    "Dockerfile" => "dockerfile",
                    "Makefile" => "makefile",
                    "Jenkinsfile" => "groovy",
                    _ => "text",
                }
            } else {
                "text"
            }
        }
    }
    .to_string()
}
