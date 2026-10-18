//! Status badges for processing, tools and approvals.
//! [Status] supplies values for the owned spans returned by [build].
use ratatui::{
    style::{Color, Style},
    text::Span,
};

/// Active approval label and total pending approvals.
/// # Examples
/// ```
/// let approval = codetether_webview_bars::prefix::Approval { id: "request", pending: 1 };
/// assert_eq!(approval.pending, 1);
/// ```
pub struct Approval<'a> {
    /// Full approval identifier; display uses its first eight bytes when valid.
    pub id: &'a str,
    /// Queue length including the active approval.
    pub pending: usize,
}
/// Borrowed state used to build status badges.
/// # Examples
/// ```
/// let status = codetether_webview_bars::prefix::Status {
///     processing: false, status: "Ready", pending_tool: None, approval: None,
/// };
/// assert!(!status.processing);
/// ```
pub struct Status<'a> {
    /// Whether a response is currently running.
    pub processing: bool,
    /// Application status text.
    pub status: &'a str,
    /// Tool currently executing, if any.
    pub pending_tool: Option<&'a str>,
    /// Current approval and queue length, if any.
    pub approval: Option<Approval<'a>>,
}

/// Build owned status spans from borrowed values.
/// # Arguments
/// * `view` - Status, tool and approval display values.
/// # Returns
/// Owned spans ready to prepend to a token summary.
/// # Examples
/// ```
/// use codetether_webview_bars::prefix::{build, Status};
/// assert!(!build(Status { processing: false, status: "Ready", pending_tool: None, approval: None }).is_empty());
/// ```
pub fn build(view: Status<'_>) -> Vec<Span<'static>> {
    let state = if view.processing {
        (" RUN ", Color::Yellow)
    } else {
        (" READY ", Color::Green)
    };
    let mut spans = vec![
        badge(state.0, state.1),
        Span::raw(" "),
        Span::styled(
            format!(" {} ", view.status),
            Style::default().fg(Color::Gray),
        ),
        Span::styled(" Ctrl+B Layout ", Style::default().fg(Color::DarkGray)),
    ];
    if let Some(tool) = view.pending_tool {
        spans.push(badge(&format!(" TOOL {tool} "), Color::Yellow));
    }
    if let Some(request) = view.approval {
        let id = request.id.get(..8).unwrap_or(request.id);
        let extra = request.pending.saturating_sub(1);
        let extra = if extra == 0 {
            String::new()
        } else {
            format!(" +{extra}")
        };
        spans.push(badge(&format!(" APPROVE Ctrl+A {id}{extra} "), Color::Red));
    }
    spans.push(Span::raw("│ "));
    spans
}

fn badge(text: &str, color: Color) -> Span<'static> {
    Span::styled(text.to_string(), Style::default().fg(color))
}
