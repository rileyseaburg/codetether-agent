//! Row rendering helpers for the Settings panel.

use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span},
};

use crate::tui::app::settings::access_mode::{current_access_mode, label};

fn setting_value_span(enabled: bool) -> Span<'static> {
    if enabled {
        "ON".green().into()
    } else {
        "OFF".yellow().into()
    }
}

fn row_style(selected: bool) -> Style {
    if selected {
        Style::default().fg(Color::Cyan).bold()
    } else {
        Style::default()
    }
}

pub(super) fn setting_line(label: &'static str, enabled: bool, selected: bool) -> Line<'static> {
    let prefix = if selected { "> " } else { "  " };
    let style = row_style(selected);
    Line::from(vec![
        Span::styled(prefix, style),
        Span::styled(format!("{label}: "), style),
        setting_value_span(enabled),
    ])
}

pub(super) fn value_line(label: &'static str, value: String, selected: bool) -> Line<'static> {
    let prefix = if selected { "> " } else { "  " };
    let style = row_style(selected);
    Line::from(vec![
        Span::styled(prefix, style),
        Span::styled(format!("{label}: "), style),
        Span::styled(value, Style::default().fg(Color::Cyan).bold()),
    ])
}

pub(super) fn access_mode_line(selected: bool) -> Line<'static> {
    value_line(
        "Access mode",
        label(current_access_mode()).to_string(),
        selected,
    )
}
