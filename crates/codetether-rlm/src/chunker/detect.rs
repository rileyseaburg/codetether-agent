//! Content-type detection.

use super::detect_helpers;
use super::types::ContentType;

/// Detect the primary type of content for optimized processing.
pub fn detect_content_type(content: &str) -> ContentType {
    let lines: Vec<&str> = content.lines().collect();
    let sample_size = lines.len().min(200);
    let sample: Vec<&str> = lines
        .iter()
        .take(sample_size / 2)
        .chain(lines.iter().rev().take(sample_size / 2))
        .copied()
        .collect();
    let (mut code, mut logs, mut convo, mut docs) = (0, 0, 0, 0);
    for line in &sample {
        let t = line.trim();
        if detect_helpers::is_code_line(t) {
            code += 1;
        }
        if detect_helpers::is_log_line(t) {
            logs += 1;
        }
        if detect_helpers::is_conversation_line(t) {
            convo += 1;
        }
        if detect_helpers::is_document_line(t) {
            docs += 1;
        }
    }
    let total = code + logs + convo + docs;
    if total == 0 {
        return ContentType::Mixed;
    }
    let thr = (total as f64 * 0.3) as usize;
    if convo > thr {
        ContentType::Conversation
    } else if logs > thr {
        ContentType::Logs
    } else if code > thr {
        ContentType::Code
    } else if docs > thr {
        ContentType::Documents
    } else {
        ContentType::Mixed
    }
}
