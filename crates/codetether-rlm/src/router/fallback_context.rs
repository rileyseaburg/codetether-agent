//! Build a structured fallback summary from preserved context.

mod files;
mod sections;
mod spans;
use files::collect_file_lines;
use sections::append_sections;

/// Build a structured fallback for `session_context` compaction output.
pub fn session_context_fallback(output: &str, input_tokens: usize) -> String {
    let lines: Vec<&str> = output.lines().collect();
    let files = collect_file_lines(&lines);
    let tools: Vec<&str> = lines
        .iter()
        .filter(|l| l.contains("[Tool "))
        .take(10)
        .copied()
        .collect();
    let errs: Vec<&str> = lines
        .iter()
        .filter(|l| l.to_lowercase().contains("error") || l.to_lowercase().contains("failed"))
        .take(5)
        .copied()
        .collect();
    let (head, tail) = spans::head_tail(&lines);
    let mut p = vec![
        "## Context Summary (Fallback Mode)".into(),
        format!("*Original: {input_tokens} tokens - RLM processing produced insufficient output*"),
        String::new(),
    ];
    if !files.is_empty() {
        p.push(format!("**Files Mentioned:** {}", files.len()));
    }
    if !tools.is_empty() {
        p.push(format!("**Recent Tool Calls:** {}", tools.join(", ")));
    }
    if !errs.is_empty() {
        p.push("**Recent Errors:**".into());
        for e in errs {
            p.push(format!("- {}", e.chars().take(150).collect::<String>()));
        }
    }
    p.push(String::new());
    append_sections(&mut p, &head, &tail);
    p.join("\n")
}
