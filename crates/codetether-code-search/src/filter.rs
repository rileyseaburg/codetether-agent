//! filter for workspace code search.
use super::*;
impl CodeSearchTool {
    pub(crate) fn should_skip(&self, path: &std::path::Path) -> bool {
        path_has_pruned_component(path)
    }

    pub(crate) fn is_text_file(&self, path: &std::path::Path) -> bool {
        let text_exts = [
            "rs", "ts", "js", "tsx", "jsx", "py", "go", "java", "c", "cpp", "h", "hpp", "md",
            "txt", "json", "yaml", "yml", "toml", "sh", "bash", "zsh", "html", "css", "scss",
        ];
        path.extension()
            .and_then(|e| e.to_str())
            .map(|e| text_exts.contains(&e))
            .unwrap_or(false)
    }
}
