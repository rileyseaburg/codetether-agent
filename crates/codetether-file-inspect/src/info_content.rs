//! info content for filesystem inspection.
use super::*;
pub(super) async fn append(
    path_obj: &Path,
    path: &str,
    metadata: &std::fs::Metadata,
    info: &mut Vec<String>,
) {
    // For files, get additional info
    if metadata.is_file() {
        // Detect language from extension
        if let Some(ext) = path_obj.extension() {
            let lang = language::name(ext);
            info.push(format!("Language: {}", lang));
        }

        // Try to read and count lines
        if let Ok(content) = fs::read_to_string(path).await {
            let lines = content.lines().count();
            let chars = content.chars().count();
            let words = content.split_whitespace().count();

            info.push(format!("Lines: {}", lines));
            info.push(format!("Words: {}", words));
            info.push(format!("Characters: {}", chars));

            // Check if it looks like UTF-8 text
            info.push("Encoding: UTF-8 (text)".to_string());
        } else {
            info.push("Encoding: Binary or non-UTF-8".to_string());
        }
    }
}
