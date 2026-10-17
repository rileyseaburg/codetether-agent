//! Local-provider strip inline think blocks.
use super::*;
impl LocalCudaPrompt {
    /// Remove inline think blocks while preserving visible response text.
    pub fn strip_inline_think_blocks(raw: &str) -> String {
        let mut remaining = raw;
        let mut visible = String::new();

        while let Some(start) = remaining.find("<think>") {
            visible.push_str(&remaining[..start]);
            let after_start = &remaining[start + "<think>".len()..];
            if let Some(end) = after_start.find("</think>") {
                remaining = &after_start[end + "</think>".len()..];
            } else {
                remaining = "";
                break;
            }
        }

        visible.push_str(remaining);
        visible.trim().to_string()
    }
}
