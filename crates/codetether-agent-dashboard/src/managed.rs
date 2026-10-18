//! Single-row formatting for the sub-agent dashboard.

use ratatui::style::Stylize;
use ratatui::text::{Line, Span};

/// Borrowed display fields for a managed child agent.
/// # Examples
/// ```
/// let agent = codetether_agent_dashboard::managed::Agent {
///     name: "reviewer", parent: None, depth: 0, instructions: "Review code",
///     model_id: None, is_processing: false,
/// };
/// assert_eq!(agent.name, "reviewer");
/// ```
#[derive(Clone, Copy)]
pub struct Agent<'a> {
    /// Child display name.
    pub name: &'a str,
    /// Parent name, defaulting to main when absent.
    pub parent: Option<&'a str>,
    /// Nesting depth controlling indentation.
    pub depth: u8,
    /// Child instructions shown as its mission.
    pub instructions: &'a str,
    /// Selected model when known.
    pub model_id: Option<&'a str>,
    /// Whether the child is processing a request.
    pub is_processing: bool,
}

/// Render one child agent row.
/// Format a child or worker dashboard row.
/// # Arguments
/// * `agent` - Borrowed managed-child fields.
/// * `selected` - Whether to show the selection marker.
/// # Returns
/// An owned display row.
/// # Examples
/// ```no_run
/// fn row(agent: &codetether_agent_dashboard::managed::Agent<'_>) {
///     let _row = codetether_agent_dashboard::managed::line(agent, false);
/// }
/// ```
pub fn line(agent: &Agent<'_>, selected: bool) -> Line<'static> {
    let parent = agent.parent.unwrap_or("main");
    let state = if agent.is_processing {
        "working"
    } else {
        "idle"
    };
    Line::from(vec![
        Span::raw(if selected { "› " } else { "  " }),
        Span::raw("  ".repeat(agent.depth as usize)),
        format!("{} ", agent.name).cyan().bold(),
        format!("← {parent} ").dim(),
        format!("[{state}] ").yellow(),
        summary(agent).dim(),
    ])
}

fn summary(agent: &Agent<'_>) -> String {
    let model = agent.model_id.unwrap_or("default model");
    let mission = if agent.instructions.is_empty() {
        "detached thread"
    } else {
        agent.instructions
    };
    format!("{model} · {mission}")
}
