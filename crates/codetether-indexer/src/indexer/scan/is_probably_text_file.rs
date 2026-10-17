//! is probably text file for codebase indexing.

use super::*;

pub(in crate::indexer) fn is_probably_text_file(path: &Path) -> bool {
    let text_exts = [
        "rs", "ts", "js", "tsx", "jsx", "py", "go", "java", "kt", "c", "cpp", "h", "hpp", "md",
        "txt", "json", "yaml", "yml", "toml", "sh", "bash", "zsh", "html", "css", "scss", "sql",
        "proto", "xml", "ini", "env", "lock",
    ];

    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        if text_exts.contains(&ext) {
            return true;
        }
    }

    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        return matches!(name, "Dockerfile" | "Makefile" | "Jenkinsfile" | "README");
    }

    false
}
