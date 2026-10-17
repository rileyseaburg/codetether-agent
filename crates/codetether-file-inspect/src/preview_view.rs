//! preview view for filesystem inspection.
use super::*;
pub(super) fn render(
    path: &str,
    content: &str,
    head_lines: usize,
    tail_lines: usize,
) -> ToolResult {
    let lines: Vec<&str> = content.lines().collect();
    let total_lines = lines.len();

    let mut output = Vec::new();
    output.push(format!("=== {} ({} lines total) ===", path, total_lines));
    output.push(String::new());

    // Head
    if head_lines > 0 {
        output.push(format!(
            "--- First {} lines ---",
            head_lines.min(total_lines)
        ));
        for (i, line) in lines.iter().take(head_lines).enumerate() {
            output.push(format!("{:4} | {}", i + 1, line));
        }
    }

    // Check if there's a gap between head and tail
    let head_end = head_lines;
    let tail_start = total_lines.saturating_sub(tail_lines);

    if tail_lines > 0 && tail_start > head_end {
        output.push(String::new());
        output.push(format!("... ({} lines omitted) ...", tail_start - head_end));
        output.push(String::new());
        output.push(format!(
            "--- Last {} lines ---",
            tail_lines.min(total_lines)
        ));
        for (i, line) in lines.iter().skip(tail_start).enumerate() {
            output.push(format!("{:4} | {}", tail_start + i + 1, line));
        }
    } else if tail_lines > 0 && tail_start <= head_end {
        // Overlap or contiguous - just show everything
        if head_end < total_lines {
            for (i, line) in lines.iter().skip(head_end).enumerate() {
                output.push(format!("{:4} | {}", head_end + i + 1, line));
            }
        }
    }

    ToolResult::success(output.join("\n")).with_metadata("total_lines", json!(total_lines))
}
