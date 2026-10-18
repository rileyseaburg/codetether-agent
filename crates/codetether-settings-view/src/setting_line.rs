//! Settings setting line.
use super::{row_style::row_style, setting_value_span::setting_value_span};
use ratatui::text::{Line, Span};

pub(super) fn setting_line(label: &'static str, enabled: bool, selected: bool) -> Line<'static> {
    let prefix = if selected { "> " } else { "  " };
    let style = row_style(selected);
    Line::from(vec![
        Span::styled(prefix, style),
        Span::styled(format!("{label}: "), style),
        setting_value_span(enabled),
    ])
}
