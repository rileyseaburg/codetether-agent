//! Bordered Webview conversation header.
//! [Header] supplies labels and [render] draws them into a frame.
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

/// Borrowed labels and message count displayed in the Webview header.
/// # Examples
/// ```
/// let header = codetether_webview_bars::header::Header {
///     session: None, agent: None, branch: None, message_count: 0,
/// };
/// assert_eq!(header.message_count, 0);
/// ```
#[derive(Clone, Copy)]
pub struct Header<'a> {
    /// Session label, or the new-session fallback when absent.
    pub session: Option<&'a str>,
    /// Active agent label, defaulting to the main agent.
    pub agent: Option<&'a str>,
    /// Current Git branch, when known.
    pub branch: Option<&'a str>,
    /// Number of visible conversation messages.
    pub message_count: usize,
}

/// Draw the header with its border and width-limited text.
/// # Arguments
/// * `f` - Destination frame.
/// * `header` - Borrowed display values.
/// * `area` - Header rectangle.
/// # Returns
/// Draws into the supplied frame.
/// # Examples
/// ```no_run
/// # use codetether_webview_bars::header::{Header, render};
/// # let mut terminal = ratatui::Terminal::new(ratatui::backend::TestBackend::new(100, 3)).unwrap();
/// terminal.draw(|frame| {
///     let area = frame.area();
///     render(frame, Header { session: None, agent: None, branch: None, message_count: 0 }, area);
/// }).unwrap();
/// ```
pub fn render(f: &mut Frame, header: Header<'_>, area: Rect) {
    let line = Line::from(header_text(header, area.width));
    let block = Block::default()
        .title(" CodeTether Webview ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let para = Paragraph::new(line)
        .block(block)
        .style(Style::default().fg(Color::Gray));
    f.render_widget(para, area);
}

fn header_text(header: Header<'_>, width: u16) -> String {
    let session = header.session.unwrap_or("#new");
    let agent = header.agent.unwrap_or("main");
    let branch = header.branch.unwrap_or("no git");
    let text = format!(
        "Workspace Chat {session} | {agent} | {branch} | {} msgs",
        header.message_count
    );
    codetether_util::truncate_bytes_safe(&text, width.saturating_sub(4) as usize).to_string()
}
