//! Rendering for an agent-tool child's in-flight event trace.

use codetether_agent_trace::{LiveTraceEntry, LiveTraceSnapshot};
use ratatui::style::Stylize;
use ratatui::text::Line;

/// Append a live child-agent trace to detail-pane rows.
///
/// # Arguments
///
/// * `rows` - Existing detail rows receiving the live section.
/// * `trace` - In-flight trace snapshot to render.
///
/// # Returns
/// Appends live activity, events and streaming text.
///
/// # Examples
///
/// ```
/// let mut rows = Vec::new();
/// let trace = codetether_agent_trace::LiveTraceSnapshot { prompt: "hello".into(), entries: vec![], streaming_text: None, activity: None };
/// codetether_agent_dashboard::live::append(&mut rows, &trace);
/// assert!(!rows.is_empty());
/// ```
pub fn append(rows: &mut Vec<Line<'static>>, trace: &LiveTraceSnapshot) {
    rows.push(Line::from("LIVE".green().bold()));
    if let Some(activity) = &trace.activity {
        rows.push(Line::from(format!("  {activity}").green()));
    }
    push_block(rows, "USER", &trace.prompt);
    for entry in &trace.entries {
        match entry {
            LiveTraceEntry::Assistant(text) => push_block(rows, "ASSISTANT", text),
            LiveTraceEntry::ToolCall { name, arguments } => {
                push_block(rows, &format!("TOOL · {name}"), arguments)
            }
            LiveTraceEntry::ToolResult {
                name,
                output,
                success,
            } => {
                let status = if *success { "ok" } else { "failed" };
                push_block(rows, &format!("TOOL RESULT · {name} · {status}"), output);
            }
            LiveTraceEntry::Error(error) => push_block(rows, "ERROR", error),
        }
    }
    if let Some(text) = &trace.streaming_text {
        push_block(rows, "ASSISTANT · streaming", text);
    }
}

fn push_block(rows: &mut Vec<Line<'static>>, label: &str, text: &str) {
    rows.push(Line::from(label.to_string().bold()));
    rows.extend(text.lines().map(|line| Line::from(format!("  {line}"))));
    rows.push(Line::from(""));
}

#[cfg(test)]
#[path = "live_tests.rs"]
mod tests;
