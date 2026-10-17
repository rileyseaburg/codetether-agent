//! estimate symbol hints for codebase indexing.

use super::*;

pub(in crate::indexer) fn estimate_symbol_hints(path: &Path, content: &str) -> u32 {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    let mut count = 0u32;
    for line in content.lines().map(str::trim_start) {
        let hit = match ext.as_str() {
            "rs" => estimate_rust_symbol_hint(line),
            "py" => line.starts_with("def ") || line.starts_with("class "),
            "ts" | "tsx" | "js" | "jsx" => {
                line.starts_with("function ")
                    || line.contains("=>")
                    || line.starts_with("class ")
                    || line.starts_with("export function ")
            }
            "go" => line.starts_with("func ") || line.starts_with("type "),
            "java" | "kt" => {
                line.contains(" class ")
                    || line.starts_with("class ")
                    || line.starts_with("interface ")
                    || line.contains(" fun ")
            }
            _ => false,
        };

        if hit {
            count = count.saturating_add(1);
        }
    }

    count
}
