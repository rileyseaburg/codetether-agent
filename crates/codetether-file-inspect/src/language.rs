//! language for filesystem inspection.
use super::*;
pub(super) fn name(ext: &std::ffi::OsStr) -> &'static str {
    match ext.to_str().unwrap_or("") {
        "rs" => "Rust",
        "py" => "Python",
        "js" => "JavaScript",
        "ts" => "TypeScript",
        "tsx" => "TypeScript (React)",
        "jsx" => "JavaScript (React)",
        "go" => "Go",
        "java" => "Java",
        "c" | "h" => "C",
        "cpp" | "hpp" | "cc" | "cxx" => "C++",
        "rb" => "Ruby",
        "php" => "PHP",
        "swift" => "Swift",
        "kt" | "kts" => "Kotlin",
        "scala" => "Scala",
        "cs" => "C#",
        "md" => "Markdown",
        "json" => "JSON",
        "yaml" | "yml" => "YAML",
        "toml" => "TOML",
        "xml" => "XML",
        "html" => "HTML",
        "css" => "CSS",
        "scss" | "sass" => "SCSS/Sass",
        "sql" => "SQL",
        "sh" | "bash" | "zsh" => "Shell",
        _ => "Unknown",
    }
}
